//! HYB_GGA_X_CAM_S12 kernel -- incremental derivative structure.

//! unpol: preamble=75 lines
//!   exc: shared=0, delta=75, outputs=1
//!   vxc: shared=75, delta=73, outputs=3
//!   fxc: shared=148, delta=147, outputs=6
//!   kxc: shared=295, delta=223, outputs=10
//!   lxc: shared=518, delta=195, outputs=15
//! pol: preamble=144 lines
//!   exc: shared=0, delta=144, outputs=1
//!   vxc: shared=144, delta=206, outputs=6
//!   fxc: shared=350, delta=544, outputs=21
//!   kxc: shared=894, delta=1040, outputs=56
//!   lxc: shared=1934, delta=1422, outputs=126

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
