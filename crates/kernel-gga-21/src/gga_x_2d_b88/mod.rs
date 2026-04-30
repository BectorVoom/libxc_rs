//! GGA_X_2D_B88 kernel -- incremental derivative structure.

//! unpol: preamble=27 lines
//!   exc: shared=0, delta=27, outputs=1
//!   vxc: shared=27, delta=22, outputs=3
//!   fxc: shared=49, delta=40, outputs=6
//!   kxc: shared=89, delta=55, outputs=10
//!   lxc: shared=144, delta=31, outputs=15
//! pol: preamble=60 lines
//!   exc: shared=0, delta=60, outputs=1
//!   vxc: shared=60, delta=68, outputs=6
//!   fxc: shared=128, delta=144, outputs=21
//!   kxc: shared=272, delta=249, outputs=56
//!   lxc: shared=521, delta=269, outputs=126

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
