//! MGGA_C_VSXC kernel -- incremental derivative structure.

//! unpol: preamble=131 lines
//!   exc: shared=0, delta=131, outputs=1
//!   vxc: shared=131, delta=156, outputs=5
//!   fxc: shared=287, delta=321, outputs=15
//!   kxc: shared=608, delta=581, outputs=35
//!   lxc: shared=1189, delta=362, outputs=70
//! pol: preamble=212 lines
//!   exc: shared=0, delta=212, outputs=1
//!   vxc: shared=212, delta=380, outputs=10
//!   fxc: shared=592, delta=1079, outputs=55
//!   kxc: shared=1671, delta=2774, outputs=220
//!   lxc: shared=4445, delta=2888, outputs=715

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
