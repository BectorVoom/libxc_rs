//! MGGA_X_PBE_GX kernel -- incremental derivative structure.

//! unpol: preamble=50 lines
//!   exc: shared=0, delta=50, outputs=1
//!   vxc: shared=50, delta=69, outputs=5
//!   fxc: shared=119, delta=172, outputs=15
//!   kxc: shared=291, delta=372, outputs=35
//!   lxc: shared=663, delta=301, outputs=70
//! pol: preamble=91 lines
//!   exc: shared=0, delta=91, outputs=1
//!   vxc: shared=91, delta=145, outputs=10
//!   fxc: shared=236, delta=431, outputs=55
//!   kxc: shared=667, delta=1061, outputs=220
//!   lxc: shared=1728, delta=1372, outputs=715

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
