//! GGA_X_PBE kernel -- incremental derivative structure.

//! unpol: preamble=26 lines
//!   exc: shared=0, delta=26, outputs=1
//!   vxc: shared=26, delta=13, outputs=3
//!   fxc: shared=39, delta=22, outputs=6
//!   kxc: shared=61, delta=28, outputs=10
//!   lxc: shared=89, delta=14, outputs=15
//! pol: preamble=51 lines
//!   exc: shared=0, delta=51, outputs=1
//!   vxc: shared=51, delta=53, outputs=6
//!   fxc: shared=104, delta=116, outputs=21
//!   kxc: shared=220, delta=216, outputs=56
//!   lxc: shared=436, delta=292, outputs=126

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
