use rust_life::GameOfLife;
use std::{thread, time};

fn main() {
    let mut game = GameOfLife::new(75, 35);

    let delay = time::Duration::from_millis(200);
    let mut step: usize = 0;

    loop {
        step += 1;
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("Step: {}", step);
        game.print();

        thread::sleep(delay);
        game.calculate_next_step();
    }
}
