//! MGGA_X_REGTPSS kernel -- incremental derivative structure.

//! unpol: preamble=99 lines
//!   exc: shared=0, delta=99, outputs=1
//!   vxc: shared=99, delta=139, outputs=5
//!   fxc: shared=238, delta=418, outputs=15
//!   kxc: shared=656, delta=1346, outputs=35
//!   lxc: shared=2002, delta=1668, outputs=70
//! pol: preamble=186 lines
//!   exc: shared=0, delta=186, outputs=1
//!   vxc: shared=186, delta=262, outputs=10
//!   fxc: shared=448, delta=830, outputs=55
//!   kxc: shared=1278, delta=2771, outputs=220
//!   lxc: shared=4049, delta=3805, outputs=715

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
