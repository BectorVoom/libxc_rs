//! GGA_K_LKT kernel -- incremental derivative structure.

//! unpol: preamble=41 lines
//!   exc: shared=0, delta=41, outputs=1
//!   vxc: shared=41, delta=19, outputs=3
//!   fxc: shared=60, delta=28, outputs=6
//!   kxc: shared=88, delta=34, outputs=10
//!   lxc: shared=122, delta=33, outputs=15
//! pol: preamble=69 lines
//!   exc: shared=0, delta=69, outputs=1
//!   vxc: shared=69, delta=60, outputs=6
//!   fxc: shared=129, delta=115, outputs=21
//!   kxc: shared=244, delta=198, outputs=56
//!   lxc: shared=442, delta=272, outputs=126

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
