//! GGA_X_2D_B86_MGC kernel -- incremental derivative structure.

//! unpol: preamble=25 lines
//!   exc: shared=0, delta=25, outputs=1
//!   vxc: shared=25, delta=16, outputs=3
//!   fxc: shared=41, delta=22, outputs=6
//!   kxc: shared=63, delta=22, outputs=10
//!   lxc: shared=85, delta=14, outputs=15
//! pol: preamble=56 lines
//!   exc: shared=0, delta=56, outputs=1
//!   vxc: shared=56, delta=58, outputs=6
//!   fxc: shared=114, delta=110, outputs=21
//!   kxc: shared=224, delta=183, outputs=56
//!   lxc: shared=407, delta=235, outputs=126

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
