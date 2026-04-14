//! GGA_X_2D_PBE kernel -- incremental derivative structure.

//! unpol: preamble=18 lines
//!   exc: shared=0, delta=18, outputs=1
//!   vxc: shared=18, delta=10, outputs=3
//!   fxc: shared=28, delta=17, outputs=6
//!   kxc: shared=45, delta=20, outputs=10
//!   lxc: shared=65, delta=13, outputs=15
//! pol: preamble=46 lines
//!   exc: shared=0, delta=46, outputs=1
//!   vxc: shared=46, delta=53, outputs=6
//!   fxc: shared=99, delta=119, outputs=21
//!   kxc: shared=218, delta=210, outputs=56
//!   lxc: shared=428, delta=273, outputs=126

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
