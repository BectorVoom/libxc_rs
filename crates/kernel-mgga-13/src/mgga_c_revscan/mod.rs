//! MGGA_C_REVSCAN kernel -- incremental derivative structure.

//! unpol: preamble=142 lines
//!   exc: shared=0, delta=142, outputs=1
//!   vxc: shared=142, delta=163, outputs=5
//!   fxc: shared=305, delta=407, outputs=15
//!   kxc: shared=712, delta=790, outputs=35
//!   lxc: shared=1502, delta=723, outputs=70
//! pol: preamble=199 lines
//!   exc: shared=0, delta=199, outputs=1
//!   vxc: shared=199, delta=337, outputs=10
//!   fxc: shared=536, delta=1198, outputs=55
//!   kxc: shared=1734, delta=3924, outputs=220
//!   lxc: shared=5658, delta=9148, outputs=715

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
