//! MGGA_K_GEA4 kernel -- incremental derivative structure.

//! unpol: preamble=47 lines
//!   exc: shared=0, delta=47, outputs=1
//!   vxc: shared=47, delta=19, outputs=5
//!   fxc: shared=66, delta=30, outputs=15
//!   kxc: shared=96, delta=38, outputs=35
//!   lxc: shared=134, delta=42, outputs=70
//! pol: preamble=74 lines
//!   exc: shared=0, delta=74, outputs=1
//!   vxc: shared=74, delta=58, outputs=10
//!   fxc: shared=132, delta=151, outputs=55
//!   kxc: shared=283, delta=352, outputs=220
//!   lxc: shared=635, delta=709, outputs=715

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
