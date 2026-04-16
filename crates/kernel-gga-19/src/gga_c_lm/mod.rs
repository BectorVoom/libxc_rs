//! GGA_C_LM kernel -- incremental derivative structure.

//! unpol: preamble=55 lines
//!   exc: shared=0, delta=55, outputs=1
//!   vxc: shared=55, delta=36, outputs=3
//!   fxc: shared=91, delta=52, outputs=6
//!   kxc: shared=143, delta=59, outputs=10
//!   lxc: shared=202, delta=25, outputs=15
//! pol: preamble=88 lines
//!   exc: shared=0, delta=88, outputs=1
//!   vxc: shared=88, delta=84, outputs=6
//!   fxc: shared=172, delta=193, outputs=21
//!   kxc: shared=365, delta=374, outputs=56
//!   lxc: shared=739, delta=603, outputs=126

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
