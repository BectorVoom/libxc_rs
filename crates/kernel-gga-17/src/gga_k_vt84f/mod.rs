//! GGA_K_VT84F kernel -- incremental derivative structure.

//! unpol: preamble=81 lines
//!   exc: shared=0, delta=81, outputs=1
//!   vxc: shared=81, delta=41, outputs=3
//!   fxc: shared=122, delta=75, outputs=6
//!   kxc: shared=197, delta=145, outputs=10
//!   lxc: shared=342, delta=199, outputs=15
//! pol: preamble=130 lines
//!   exc: shared=0, delta=130, outputs=1
//!   vxc: shared=130, delta=102, outputs=6
//!   fxc: shared=232, delta=206, outputs=21
//!   kxc: shared=438, delta=422, outputs=56
//!   lxc: shared=860, delta=615, outputs=126

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
