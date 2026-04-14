//! MGGA_K_GEA2 kernel -- incremental derivative structure.

//! unpol: preamble=32 lines
//!   exc: shared=0, delta=32, outputs=1
//!   vxc: shared=32, delta=14, outputs=5
//!   fxc: shared=46, delta=18, outputs=15
//!   kxc: shared=64, delta=28, outputs=35
//!   lxc: shared=92, delta=38, outputs=70
//! pol: preamble=54 lines
//!   exc: shared=0, delta=54, outputs=1
//!   vxc: shared=54, delta=55, outputs=10
//!   fxc: shared=109, delta=135, outputs=55
//!   kxc: shared=244, delta=318, outputs=220
//!   lxc: shared=562, delta=665, outputs=715

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
