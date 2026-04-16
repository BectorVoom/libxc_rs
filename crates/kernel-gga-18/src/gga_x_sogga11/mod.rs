//! GGA_X_SOGGA11 kernel -- incremental derivative structure.

//! unpol: preamble=52 lines
//!   exc: shared=0, delta=52, outputs=1
//!   vxc: shared=52, delta=49, outputs=3
//!   fxc: shared=101, delta=103, outputs=6
//!   kxc: shared=204, delta=140, outputs=10
//!   lxc: shared=344, delta=172, outputs=15
//! pol: preamble=85 lines
//!   exc: shared=0, delta=85, outputs=1
//!   vxc: shared=85, delta=102, outputs=6
//!   fxc: shared=187, delta=238, outputs=21
//!   kxc: shared=425, delta=403, outputs=56
//!   lxc: shared=828, delta=537, outputs=126

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
