//! GGA_X_CAP kernel -- incremental derivative structure.

//! unpol: preamble=33 lines
//!   exc: shared=0, delta=33, outputs=1
//!   vxc: shared=33, delta=35, outputs=3
//!   fxc: shared=68, delta=50, outputs=6
//!   kxc: shared=118, delta=95, outputs=10
//!   lxc: shared=213, delta=61, outputs=15
//! pol: preamble=58 lines
//!   exc: shared=0, delta=58, outputs=1
//!   vxc: shared=58, delta=75, outputs=6
//!   fxc: shared=133, delta=154, outputs=21
//!   kxc: shared=287, delta=286, outputs=56
//!   lxc: shared=573, delta=343, outputs=126

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
