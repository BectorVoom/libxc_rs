//! GGA_C_CCDF kernel -- incremental derivative structure.

//! unpol: preamble=19 lines
//!   exc: shared=0, delta=19, outputs=1
//!   vxc: shared=19, delta=14, outputs=3
//!   fxc: shared=33, delta=53, outputs=6
//!   kxc: shared=86, delta=89, outputs=10
//!   lxc: shared=175, delta=45, outputs=15
//! pol: preamble=21 lines
//!   exc: shared=0, delta=21, outputs=1
//!   vxc: shared=21, delta=18, outputs=6
//!   fxc: shared=39, delta=72, outputs=21
//!   kxc: shared=111, delta=145, outputs=56
//!   lxc: shared=256, delta=175, outputs=126

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
