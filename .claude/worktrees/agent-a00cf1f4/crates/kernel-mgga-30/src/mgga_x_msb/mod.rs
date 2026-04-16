//! MGGA_X_MSB kernel -- incremental derivative structure.

//! unpol: preamble=56 lines
//!   exc: shared=0, delta=56, outputs=1
//!   vxc: shared=56, delta=60, outputs=5
//!   fxc: shared=116, delta=169, outputs=15
//!   kxc: shared=285, delta=438, outputs=35
//!   lxc: shared=723, delta=438, outputs=70
//! pol: preamble=101 lines
//!   exc: shared=0, delta=101, outputs=1
//!   vxc: shared=101, delta=140, outputs=10
//!   fxc: shared=241, delta=416, outputs=55
//!   kxc: shared=657, delta=1106, outputs=220
//!   lxc: shared=1763, delta=1449, outputs=715

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
