//! MGGA_C_RPPSCAN kernel -- incremental derivative structure.

//! unpol: preamble=106 lines
//!   exc: shared=0, delta=106, outputs=1
//!   vxc: shared=106, delta=136, outputs=5
//!   fxc: shared=242, delta=356, outputs=15
//!   kxc: shared=598, delta=694, outputs=35
//!   lxc: shared=1292, delta=615, outputs=70
//! pol: preamble=163 lines
//!   exc: shared=0, delta=163, outputs=1
//!   vxc: shared=163, delta=286, outputs=10
//!   fxc: shared=449, delta=1019, outputs=55
//!   kxc: shared=1468, delta=3278, outputs=220
//!   lxc: shared=4746, delta=7665, outputs=715

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
