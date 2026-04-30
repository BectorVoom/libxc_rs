//! GGA_K_EXP4 kernel -- incremental derivative structure.

//! unpol: preamble=40 lines
//!   exc: shared=0, delta=40, outputs=1
//!   vxc: shared=40, delta=18, outputs=3
//!   fxc: shared=58, delta=32, outputs=6
//!   kxc: shared=90, delta=31, outputs=10
//!   lxc: shared=121, delta=16, outputs=15
//! pol: preamble=66 lines
//!   exc: shared=0, delta=66, outputs=1
//!   vxc: shared=66, delta=44, outputs=6
//!   fxc: shared=110, delta=110, outputs=21
//!   kxc: shared=220, delta=189, outputs=56
//!   lxc: shared=409, delta=243, outputs=126

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
