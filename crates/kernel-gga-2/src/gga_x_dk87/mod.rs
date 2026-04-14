//! GGA_X_DK87 kernel -- incremental derivative structure.

//! unpol: preamble=44 lines
//!   exc: shared=0, delta=44, outputs=1
//!   vxc: shared=44, delta=29, outputs=3
//!   fxc: shared=73, delta=48, outputs=6
//!   kxc: shared=121, delta=67, outputs=10
//!   lxc: shared=188, delta=36, outputs=15
//! pol: preamble=70 lines
//!   exc: shared=0, delta=70, outputs=1
//!   vxc: shared=70, delta=70, outputs=6
//!   fxc: shared=140, delta=150, outputs=21
//!   kxc: shared=290, delta=263, outputs=56
//!   lxc: shared=553, delta=279, outputs=126

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
