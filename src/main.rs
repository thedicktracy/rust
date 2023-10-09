use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io;
use serde_json;


#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: u32,
    title: String,
    description: String,
    completed: bool,
}

impl Task {
    fn new(id: u32, title: String, description: String) -> Task {
        Task {
            id,
            title,
            description,
            completed: false,
        }
    }
}

struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    fn new() -> TaskManager {
        TaskManager { tasks: Vec::new() }
    }

    fn add_task(&mut self, title: String, description: String) {
        let id = (self.tasks.len() + 1) as u32;
        let task = Task::new(id, title, description);
        self.tasks.push(task);
    }

    fn list_tasks(&self) {
        for task in &self.tasks {
            println!(
                "[{}] Title: {} - Description: {} - Completed: {}",
                task.id, task.title, task.description, task.completed
            );
        }
    }

    fn save_tasks(&self, filename: &str) -> Result<(), serde_json::Error> {
        let file = match File::create(filename) {
            Ok(f) => f,
            Err(e) => {
                return Err(serde_json::Error::io(e));
            }
        };
        
        serde_json::to_writer_pretty(file, &self.tasks).map_err(serde_json::Error::from)?;
        Ok(())
    }
    
    
}

fn main() {
    let mut task_manager = TaskManager::new();
    let filename = "tasks.json";

    loop {
        println!("Task Manager");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Save and Exit");
        println!("Enter your choice:");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => {
                println!("Enter task title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).expect("Failed to read line");

                println!("Enter task description:");
                let mut description = String::new();
                io::stdin().read_line(&mut description).expect("Failed to read line");

                task_manager.add_task(title.trim().to_string(), description.trim().to_string());
            }
            2 => {
                task_manager.list_tasks();
            }
            3 => {
                // Save tasks to the file before exiting
                task_manager.save_tasks(filename).unwrap_or_else(|err| {
                    println!("Error saving tasks: {}", err);
                });
                println!("Tasks saved. Exiting.");
                break;
            }
            _ => {
                println!("Invalid choice. Please enter a number between 1 and 3.");
            }
        }
    }
}

