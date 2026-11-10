use core::cmp::Ordering;
use rand::Rng;
use std::io;

fn main() {
    // 数を当ててごらん
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // 秘密の数字は次の通り: {secret_number}
    println!("The secret number is: {secret_number}");

    loop {
        // ほら、予想を入力してね
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // 行の読み込みに失敗しました

        // 数値を入力してください！
        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        // 次の様に予想しました: (guess)
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),  // 小さすぎ！
            Ordering::Greater => println!("Too big!"), // 大きすぎ！
            Ordering::Equal => {
                println!("You win!"); // やったね！
                break;
            }
        }
    }
}
