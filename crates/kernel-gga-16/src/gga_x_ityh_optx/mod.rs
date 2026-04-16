//! GGA_X_ITYH_OPTX kernel -- incremental derivative structure.

//! unpol: preamble=70 lines
//!   exc: shared=0, delta=70, outputs=1
//!   vxc: shared=70, delta=64, outputs=3
//!   fxc: shared=134, delta=134, outputs=6
//!   kxc: shared=268, delta=175, outputs=10
//!   lxc: shared=443, delta=147, outputs=15
//! pol: preamble=136 lines
//!   exc: shared=0, delta=136, outputs=1
//!   vxc: shared=136, delta=194, outputs=6
//!   fxc: shared=330, delta=535, outputs=21
//!   kxc: shared=865, delta=959, outputs=56
//!   lxc: shared=1824, delta=1348, outputs=126

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
