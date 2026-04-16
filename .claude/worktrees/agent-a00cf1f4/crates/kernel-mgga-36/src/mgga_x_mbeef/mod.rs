//! MGGA_X_MBEEF kernel -- incremental derivative structure.

//! unpol: preamble=92 lines
//!   exc: shared=0, delta=92, outputs=1
//!   vxc: shared=92, delta=187, outputs=5
//!   fxc: shared=279, delta=484, outputs=15
//!   kxc: shared=763, delta=1030, outputs=35
//!   lxc: shared=1793, delta=1824, outputs=70
//! pol: preamble=167 lines
//!   exc: shared=0, delta=167, outputs=1
//!   vxc: shared=167, delta=388, outputs=10
//!   fxc: shared=555, delta=1046, outputs=55
//!   kxc: shared=1601, delta=2306, outputs=220
//!   lxc: shared=3907, delta=4289, outputs=715

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
