//! MGGA_X_R4SCAN kernel -- incremental derivative structure.

//! unpol: preamble=116 lines
//!   exc: shared=0, delta=116, outputs=1
//!   vxc: shared=116, delta=163, outputs=5
//!   fxc: shared=279, delta=527, outputs=15
//!   kxc: shared=806, delta=1507, outputs=35
//!   lxc: shared=2313, delta=1740, outputs=70
//! pol: preamble=199 lines
//!   exc: shared=0, delta=199, outputs=1
//!   vxc: shared=199, delta=341, outputs=10
//!   fxc: shared=540, delta=1126, outputs=55
//!   kxc: shared=1666, delta=3383, outputs=220
//!   lxc: shared=5049, delta=4247, outputs=715

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
