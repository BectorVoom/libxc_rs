//! GGA_X_FT97 kernel -- incremental derivative structure.

//! unpol: preamble=50 lines
//!   exc: shared=0, delta=50, outputs=1
//!   vxc: shared=50, delta=62, outputs=3
//!   fxc: shared=112, delta=117, outputs=6
//!   kxc: shared=229, delta=219, outputs=10
//!   lxc: shared=448, delta=130, outputs=15
//! pol: preamble=98 lines
//!   exc: shared=0, delta=98, outputs=1
//!   vxc: shared=98, delta=219, outputs=6
//!   fxc: shared=317, delta=743, outputs=21
//!   kxc: shared=1060, delta=2247, outputs=56
//!   lxc: shared=3307, delta=2241, outputs=126

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
