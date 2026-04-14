//! GGA_X_B86 kernel -- incremental derivative structure.

//! unpol: preamble=26 lines
//!   exc: shared=0, delta=26, outputs=1
//!   vxc: shared=26, delta=20, outputs=3
//!   fxc: shared=46, delta=34, outputs=6
//!   kxc: shared=80, delta=50, outputs=10
//!   lxc: shared=130, delta=39, outputs=15
//! pol: preamble=49 lines
//!   exc: shared=0, delta=49, outputs=1
//!   vxc: shared=49, delta=61, outputs=6
//!   fxc: shared=110, delta=131, outputs=21
//!   kxc: shared=241, delta=231, outputs=56
//!   lxc: shared=472, delta=282, outputs=126

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
