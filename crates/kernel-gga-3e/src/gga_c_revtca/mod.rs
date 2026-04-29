//! GGA_C_REVTCA kernel -- incremental derivative structure.

//! unpol: preamble=35 lines
//!   exc: shared=0, delta=35, outputs=1
//!   vxc: shared=35, delta=20, outputs=3
//!   fxc: shared=55, delta=47, outputs=6
//!   kxc: shared=102, delta=85, outputs=10
//!   lxc: shared=187, delta=66, outputs=15
//! pol: preamble=99 lines
//!   exc: shared=0, delta=99, outputs=1
//!   vxc: shared=99, delta=108, outputs=6
//!   fxc: shared=207, delta=333, outputs=21
//!   kxc: shared=540, delta=1237, outputs=56
//!   lxc: shared=1777, delta=3571, outputs=126

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
