//! MGGA_X_SCAN kernel -- incremental derivative structure.

//! unpol: preamble=86 lines
//!   exc: shared=0, delta=86, outputs=1
//!   vxc: shared=86, delta=100, outputs=5
//!   fxc: shared=186, delta=231, outputs=15
//!   kxc: shared=417, delta=468, outputs=35
//!   lxc: shared=885, delta=491, outputs=70
//! pol: preamble=156 lines
//!   exc: shared=0, delta=156, outputs=1
//!   vxc: shared=156, delta=207, outputs=10
//!   fxc: shared=363, delta=582, outputs=55
//!   kxc: shared=945, delta=1373, outputs=220
//!   lxc: shared=2318, delta=2114, outputs=715

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
