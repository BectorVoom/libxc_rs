//! MGGA_X_GVT4 kernel -- incremental derivative structure.

//! unpol: preamble=48 lines
//!   exc: shared=0, delta=48, outputs=1
//!   vxc: shared=48, delta=37, outputs=5
//!   fxc: shared=85, delta=72, outputs=15
//!   kxc: shared=157, delta=142, outputs=35
//!   lxc: shared=299, delta=125, outputs=70
//! pol: preamble=86 lines
//!   exc: shared=0, delta=86, outputs=1
//!   vxc: shared=86, delta=93, outputs=10
//!   fxc: shared=179, delta=220, outputs=55
//!   kxc: shared=399, delta=543, outputs=220
//!   lxc: shared=942, delta=868, outputs=715

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
