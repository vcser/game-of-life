//extern crate rustbox;
extern crate rand;

//use rustbox::RustBox;
use rand::prelude::*;

const ALIVE: char = '⬛'; // black box emoji
const DEAD: char = '⬜'; // white box emoji
const SPEED: u64 = 250;

fn main() {
    let mut rng = rand::thread_rng();
    let mut board = [[DEAD; 40]; 40];
    for _ in 0..800 {
        let x = rng.gen_range(0..40);
        let y = rng.gen_range(0..40);
        board[x][y] = ALIVE;
    }

    loop {
        // draw board
        std::process::Command::new("clear").status().unwrap();
        for i in board.iter() {
            for j in i {
                print!("{}", j);
            }
            println!("");
        }

        let mut alive = Vec::new();

        // check alive
        for (i, row) in board.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if i == 0 || j == 0 || i == board.len() - 1 || j == board.len() - 1 {
                    continue;
                }
                let mut neighbors = 0;
                for k in -1..2i32 {
                    for l in -1..2i32 {
                        if board[(i as i32 + k) as usize][(j as i32 + l) as usize] == ALIVE {
                            neighbors += 1;
                        }
                    }
                }
                if *cell == ALIVE && (neighbors == 3 || neighbors == 4) {
                    alive.push((i, j));
                }
                else if *cell == DEAD && (neighbors == 3){// || neighbors == 6) { // highlife
                    alive.push((i, j));
                }
            }
        }

        // put alive cells
        board = [[DEAD; 40]; 40];
        for (x, y) in alive {
            board[x][y] = ALIVE;
        }
        std::thread::sleep(std::time::Duration::from_millis(SPEED));
    }
}
