//! MGGA_K_PC07 kernel -- incremental derivative structure.

//! unpol: preamble=86 lines
//!   exc: shared=0, delta=86, outputs=1
//!   vxc: shared=86, delta=96, outputs=5
//!   fxc: shared=182, delta=316, outputs=15
//!   kxc: shared=498, delta=1023, outputs=35
//!   lxc: shared=1521, delta=2488, outputs=70
//! pol: preamble=146 lines
//!   exc: shared=0, delta=146, outputs=1
//!   vxc: shared=146, delta=203, outputs=10
//!   fxc: shared=349, delta=688, outputs=55
//!   kxc: shared=1037, delta=2268, outputs=220
//!   lxc: shared=3305, delta=5636, outputs=715

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
