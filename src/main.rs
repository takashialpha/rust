use std::io;

struct Ui {
	name: String,
	bar: String,
	screen_content: String,
	break_line: String,
}

struct Note {
	title: String,
	content: String,
}

impl Note {
	fn new(title:String,content:String) -> Note {
		Note {
			title: title.trim().to_string(),
			content: content.trim().to_string(),
		}
	}
	
}

impl Ui {
	fn new(name:&str,bar:&str,screen_content:&str,break_line:&str) -> Ui{
		Ui {
			name: name.to_string(),
			bar: bar.to_string().repeat(20),
			screen_content: screen_content.to_string(),
			break_line: break_line.to_string().repeat(20),
		}
	}
}

fn new_note() {
	let mut ui = Ui::new("New Note","+","New note name:","\n");
	println!("{}{}\n{}\n{}\n{}\n{}",ui.break_line, ui.bar, ui.name, ui.bar,ui.screen_content, ui.bar);
	let mut note_name = String::new();
	io::stdin()
	.read_line(&mut note_name)
	.expect("Failed to read user input. ");
	println!("{}",ui.bar);
	ui = Ui::new("New note","%", "Add content","\n");
	println!("{}{}\n{}\n{}",ui.break_line,ui.bar,ui.screen_content,ui.bar);
	let mut note_content = String::new();
	io::stdin()
	.read_line(&mut note_content)
	.expect("Failed to read user input");
	println!("{}",ui.bar);	
}

fn show_notes(){
	
}

fn delete_note() {
	
}

fn edit_note() {
	
}

fn main_menu() {
	let ui = Ui::new("Menu","=","1. New Note\n2. Show Notes\n3. Delete Note\n4. Edit Note\n5. Exit","\n");
	loop {
		println!("{}{}\n{}\n{}\n{}\n{}",ui.break_line,ui.bar,ui.name,ui.bar,ui.screen_content,ui.bar);
		let mut user_input = String::new();
		io::stdin()
		.read_line(&mut user_input)
		.expect("Failed to read user input. ");
		match user_input.trim() {
			"1" => new_note(),
			"2" => println!(),
			"3" => println!(),
			"4" => println!(),
			"5" => break,
			_ => println!("Please insert a valid number. "),
		}
	}
}
fn main() {
	main_menu();
}
