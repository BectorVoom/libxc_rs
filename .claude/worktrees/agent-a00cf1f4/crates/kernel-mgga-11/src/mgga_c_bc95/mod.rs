//! MGGA_C_BC95 kernel -- incremental derivative structure.

//! unpol: preamble=88 lines
//!   exc: shared=0, delta=88, outputs=1
//!   vxc: shared=88, delta=110, outputs=5
//!   fxc: shared=198, delta=212, outputs=15
//!   kxc: shared=410, delta=305, outputs=35
//!   lxc: shared=715, delta=169, outputs=70
//! pol: preamble=158 lines
//!   exc: shared=0, delta=158, outputs=1
//!   vxc: shared=158, delta=311, outputs=10
//!   fxc: shared=469, delta=859, outputs=55
//!   kxc: shared=1328, delta=1952, outputs=220
//!   lxc: shared=3280, delta=2119, outputs=715

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
