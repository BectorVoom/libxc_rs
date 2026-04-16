//! MGGA_C_REVTPSS kernel -- incremental derivative structure.

//! unpol: preamble=307 lines
//!   exc: shared=0, delta=307, outputs=1
//!   vxc: shared=307, delta=435, outputs=5
//!   fxc: shared=742, delta=1012, outputs=15
//!   kxc: shared=1754, delta=2232, outputs=35
//!   lxc: shared=3986, delta=1869, outputs=70
//! pol: preamble=382 lines
//!   exc: shared=0, delta=382, outputs=1
//!   vxc: shared=382, delta=913, outputs=10
//!   fxc: shared=1295, delta=3168, outputs=55
//!   kxc: shared=4463, delta=11280, outputs=220
//!   lxc: shared=15743, delta=26213, outputs=715

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
