//! MGGA_C_M08 kernel -- incremental derivative structure.

//! unpol: preamble=156 lines
//!   exc: shared=0, delta=156, outputs=1
//!   vxc: shared=156, delta=182, outputs=5
//!   fxc: shared=338, delta=325, outputs=15
//!   kxc: shared=663, delta=466, outputs=35
//!   lxc: shared=1129, delta=417, outputs=70
//! pol: preamble=206 lines
//!   exc: shared=0, delta=206, outputs=1
//!   vxc: shared=206, delta=331, outputs=10
//!   fxc: shared=537, delta=979, outputs=55
//!   kxc: shared=1516, delta=2831, outputs=220
//!   lxc: shared=4347, delta=5806, outputs=715

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
