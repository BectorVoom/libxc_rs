//! GGA_C_HCTH_A kernel -- incremental derivative structure.

//! unpol: preamble=108 lines
//!   exc: shared=0, delta=108, outputs=1
//!   vxc: shared=108, delta=135, outputs=3
//!   fxc: shared=243, delta=219, outputs=6
//!   kxc: shared=462, delta=353, outputs=10
//!   lxc: shared=815, delta=225, outputs=15
//! pol: preamble=194 lines
//!   exc: shared=0, delta=194, outputs=1
//!   vxc: shared=194, delta=347, outputs=6
//!   fxc: shared=541, delta=904, outputs=21
//!   kxc: shared=1445, delta=2521, outputs=56
//!   lxc: shared=3966, delta=3580, outputs=126

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
