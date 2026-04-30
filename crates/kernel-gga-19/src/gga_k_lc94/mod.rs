//! GGA_K_LC94 kernel -- incremental derivative structure.

//! unpol: preamble=49 lines
//!   exc: shared=0, delta=49, outputs=1
//!   vxc: shared=49, delta=47, outputs=3
//!   fxc: shared=96, delta=65, outputs=6
//!   kxc: shared=161, delta=97, outputs=10
//!   lxc: shared=258, delta=66, outputs=15
//! pol: preamble=84 lines
//!   exc: shared=0, delta=84, outputs=1
//!   vxc: shared=84, delta=104, outputs=6
//!   fxc: shared=188, delta=177, outputs=21
//!   kxc: shared=365, delta=324, outputs=56
//!   lxc: shared=689, delta=333, outputs=126

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
