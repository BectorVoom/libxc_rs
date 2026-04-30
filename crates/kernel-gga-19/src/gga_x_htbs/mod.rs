//! GGA_X_HTBS kernel -- incremental derivative structure.

//! unpol: preamble=70 lines
//!   exc: shared=0, delta=70, outputs=1
//!   vxc: shared=70, delta=44, outputs=3
//!   fxc: shared=114, delta=79, outputs=6
//!   kxc: shared=193, delta=104, outputs=10
//!   lxc: shared=297, delta=75, outputs=15
//! pol: preamble=116 lines
//!   exc: shared=0, delta=116, outputs=1
//!   vxc: shared=116, delta=91, outputs=6
//!   fxc: shared=207, delta=199, outputs=21
//!   kxc: shared=406, delta=339, outputs=56
//!   lxc: shared=745, delta=367, outputs=126

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
