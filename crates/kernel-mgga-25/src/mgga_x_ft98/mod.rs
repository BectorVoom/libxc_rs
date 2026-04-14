//! MGGA_X_FT98 kernel -- incremental derivative structure.

//! unpol: preamble=102 lines
//!   exc: shared=0, delta=102, outputs=1
//!   vxc: shared=102, delta=120, outputs=5
//!   fxc: shared=222, delta=350, outputs=15
//!   kxc: shared=572, delta=977, outputs=35
//!   lxc: shared=1549, delta=949, outputs=70
//! pol: preamble=176 lines
//!   exc: shared=0, delta=176, outputs=1
//!   vxc: shared=176, delta=254, outputs=10
//!   fxc: shared=430, delta=801, outputs=55
//!   kxc: shared=1231, delta=2231, outputs=220
//!   lxc: shared=3462, delta=2548, outputs=715

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
