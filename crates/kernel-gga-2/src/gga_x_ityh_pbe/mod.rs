//! GGA_X_ITYH_PBE kernel -- incremental derivative structure.

//! unpol: preamble=69 lines
//!   exc: shared=0, delta=69, outputs=1
//!   vxc: shared=69, delta=65, outputs=3
//!   fxc: shared=134, delta=157, outputs=6
//!   kxc: shared=291, delta=218, outputs=10
//!   lxc: shared=509, delta=191, outputs=15
//! pol: preamble=130 lines
//!   exc: shared=0, delta=130, outputs=1
//!   vxc: shared=130, delta=210, outputs=6
//!   fxc: shared=340, delta=598, outputs=21
//!   kxc: shared=938, delta=1099, outputs=56
//!   lxc: shared=2037, delta=1546, outputs=126

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
