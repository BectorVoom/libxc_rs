//! GGA_X_BAYESIAN kernel -- incremental derivative structure.

//! unpol: preamble=39 lines
//!   exc: shared=0, delta=39, outputs=1
//!   vxc: shared=39, delta=28, outputs=3
//!   fxc: shared=67, delta=47, outputs=6
//!   kxc: shared=114, delta=72, outputs=10
//!   lxc: shared=186, delta=54, outputs=15
//! pol: preamble=66 lines
//!   exc: shared=0, delta=66, outputs=1
//!   vxc: shared=66, delta=74, outputs=6
//!   fxc: shared=140, delta=154, outputs=21
//!   kxc: shared=294, delta=285, outputs=56
//!   lxc: shared=579, delta=303, outputs=126

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
