//! GGA_X_LV_RPW86 kernel -- incremental derivative structure.

//! unpol: preamble=48 lines
//!   exc: shared=0, delta=48, outputs=1
//!   vxc: shared=48, delta=44, outputs=3
//!   fxc: shared=92, delta=66, outputs=6
//!   kxc: shared=158, delta=104, outputs=10
//!   lxc: shared=262, delta=86, outputs=15
//! pol: preamble=84 lines
//!   exc: shared=0, delta=84, outputs=1
//!   vxc: shared=84, delta=103, outputs=6
//!   fxc: shared=187, delta=183, outputs=21
//!   kxc: shared=370, delta=335, outputs=56
//!   lxc: shared=705, delta=379, outputs=126

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
