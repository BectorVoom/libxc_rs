//! GGA_X_KT kernel -- incremental derivative structure.

//! unpol: preamble=36 lines
//!   exc: shared=0, delta=36, outputs=1
//!   vxc: shared=36, delta=22, outputs=3
//!   fxc: shared=58, delta=25, outputs=6
//!   kxc: shared=83, delta=31, outputs=10
//!   lxc: shared=114, delta=9, outputs=15
//! pol: preamble=67 lines
//!   exc: shared=0, delta=67, outputs=1
//!   vxc: shared=67, delta=79, outputs=6
//!   fxc: shared=146, delta=198, outputs=21
//!   kxc: shared=344, delta=500, outputs=56
//!   lxc: shared=844, delta=509, outputs=126

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
