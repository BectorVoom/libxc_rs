//! MGGA_X_TM kernel -- incremental derivative structure.

//! unpol: preamble=69 lines
//!   exc: shared=0, delta=69, outputs=1
//!   vxc: shared=69, delta=87, outputs=5
//!   fxc: shared=156, delta=207, outputs=15
//!   kxc: shared=363, delta=592, outputs=35
//!   lxc: shared=955, delta=693, outputs=70
//! pol: preamble=123 lines
//!   exc: shared=0, delta=123, outputs=1
//!   vxc: shared=123, delta=182, outputs=10
//!   fxc: shared=305, delta=490, outputs=55
//!   kxc: shared=795, delta=1417, outputs=220
//!   lxc: shared=2212, delta=1987, outputs=715

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
