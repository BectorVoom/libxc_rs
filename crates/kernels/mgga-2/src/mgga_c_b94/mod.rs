//! MGGA_C_B94 kernel — split into per-function files.
//!
//! kxc_pol and kxc_unpol are aggregated into single `#[cube] fn` entry points
//! that delegate to their respective partN private internal modules
//! (`kxc_pol/part0..part15`, `kxc_unpol/part0..part2`). This restructure
//! is the Plan 1 hypothesis test from quick task 260514-q02 — moving the
//! partitioned files under a single function module rather than holding
//! them as flat siblings of mod.rs.

pub mod exc_unpol;
pub mod vxc_unpol;
pub mod fxc_unpol;
pub mod kxc_unpol;
pub mod exc_pol;
pub mod vxc_pol;
pub mod fxc_pol;
pub mod kxc_pol;
