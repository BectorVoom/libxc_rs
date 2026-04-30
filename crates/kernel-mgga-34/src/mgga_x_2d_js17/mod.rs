//! MGGA_X_2D_JS17 kernel -- incremental derivative structure.

//! unpol: preamble=28 lines
//!   exc: shared=0, delta=28, outputs=1
//!   vxc: shared=28, delta=23, outputs=5
//!   fxc: shared=51, delta=39, outputs=15
//!   kxc: shared=90, delta=66, outputs=35
//!   lxc: shared=156, delta=72, outputs=70
//! pol: preamble=65 lines
//!   exc: shared=0, delta=65, outputs=1
//!   vxc: shared=65, delta=71, outputs=10
//!   fxc: shared=136, delta=181, outputs=55
//!   kxc: shared=317, delta=422, outputs=220
//!   lxc: shared=739, delta=798, outputs=715

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
