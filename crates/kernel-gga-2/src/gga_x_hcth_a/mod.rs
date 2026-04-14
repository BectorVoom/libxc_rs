//! GGA_X_HCTH_A kernel -- incremental derivative structure.

//! unpol: preamble=35 lines
//!   exc: shared=0, delta=35, outputs=1
//!   vxc: shared=35, delta=23, outputs=3
//!   fxc: shared=58, delta=44, outputs=6
//!   kxc: shared=102, delta=54, outputs=10
//!   lxc: shared=156, delta=32, outputs=15
//! pol: preamble=64 lines
//!   exc: shared=0, delta=64, outputs=1
//!   vxc: shared=64, delta=69, outputs=6
//!   fxc: shared=133, delta=142, outputs=21
//!   kxc: shared=275, delta=235, outputs=56
//!   lxc: shared=510, delta=269, outputs=126

pub mod exc_unpol;
pub mod vxc_unpol;
pub mod fxc_unpol;
pub mod kxc_unpol;
pub mod lxc_unpol;
pub mod exc_pol;
pub mod vxc_pol;
pub mod fxc_pol;
pub mod kxc_pol;
pub mod lxc_pol;
