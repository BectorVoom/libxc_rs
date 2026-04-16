//! MGGA_C_PKZB kernel -- incremental derivative structure.

//! unpol: preamble=143 lines
//!   exc: shared=0, delta=143, outputs=1
//!   vxc: shared=143, delta=173, outputs=5
//!   fxc: shared=316, delta=358, outputs=15
//!   kxc: shared=674, delta=607, outputs=35
//!   lxc: shared=1281, delta=417, outputs=70
//! pol: preamble=271 lines
//!   exc: shared=0, delta=271, outputs=1
//!   vxc: shared=271, delta=534, outputs=10
//!   fxc: shared=805, delta=1718, outputs=55
//!   kxc: shared=2523, delta=5516, outputs=220
//!   lxc: shared=8039, delta=9133, outputs=715

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
