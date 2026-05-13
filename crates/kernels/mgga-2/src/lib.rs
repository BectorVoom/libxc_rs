#![allow(clippy::excessive_precision)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::too_many_arguments)]

//! MGGA kernel translations batch 2.

// `mgga_c_b94` is in the runtime deferred-kernels list
// (`crates/kernels/mgga/src/deferred.rs`, libxc id 397). `MggaFunctional::from_id`
// rejects it before any kernel module is touched, so it is dead code at
// runtime but the proc-macro expansion of its 26 `#[cube] fn`s (~72k lines)
// is paid on every `cargo check`. The `deferred-kernels` feature opts the
// module back in for completeness; the default omits it so build-time RSS
// stays down. Quick task 260514-q02 design memo for rationale.
#[cfg(feature = "deferred-kernels")]
pub mod mgga_c_b94;
pub mod mgga_c_rppscan;
pub mod mgga_c_rregtm;
pub mod mgga_c_scan;
pub mod mgga_k_csk_loc;
pub mod mgga_k_pc07;
pub mod mgga_x_edmgga;
pub mod mgga_x_m11;
pub mod mgga_x_mcml;
pub mod mgga_x_mn12;
pub mod mgga_x_r2scan;
pub mod mgga_x_rppscan;
pub mod mgga_x_rtpss;
pub mod mgga_x_task;
pub mod mgga_x_tm;
pub mod mgga_x_tpss;
pub mod mgga_x_vcml;
