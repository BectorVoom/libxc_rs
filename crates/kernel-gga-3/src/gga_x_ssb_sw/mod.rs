//! GGA_X_SSB_SW kernel -- incremental derivative structure.

//! unpol: preamble=39 lines
//!   exc: shared=0, delta=39, outputs=1
//!   vxc: shared=39, delta=32, outputs=3
//!   fxc: shared=71, delta=41, outputs=6
//!   kxc: shared=112, delta=52, outputs=10
//!   lxc: shared=164, delta=22, outputs=15
//! pol: preamble=74 lines
//!   exc: shared=0, delta=74, outputs=1
//!   vxc: shared=74, delta=79, outputs=6
//!   fxc: shared=153, delta=138, outputs=21
//!   kxc: shared=291, delta=235, outputs=56
//!   lxc: shared=526, delta=246, outputs=126

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
