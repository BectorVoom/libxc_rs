//! MGGA_X_MBEEFVDW kernel -- incremental derivative structure.

//! unpol: preamble=75 lines
//!   exc: shared=0, delta=75, outputs=1
//!   vxc: shared=75, delta=120, outputs=5
//!   fxc: shared=195, delta=297, outputs=15
//!   kxc: shared=492, delta=662, outputs=35
//!   lxc: shared=1154, delta=1055, outputs=70
//! pol: preamble=133 lines
//!   exc: shared=0, delta=133, outputs=1
//!   vxc: shared=133, delta=254, outputs=10
//!   fxc: shared=387, delta=672, outputs=55
//!   kxc: shared=1059, delta=1578, outputs=220
//!   lxc: shared=2637, delta=2761, outputs=715

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
