//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1373/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1373(t1888: f64, t232: f64, t6646: f64, t68217: f64, t105574: f64, t105578: f64, t105582: f64, t105586: f64, t105596: f64, t25255: f64, t5612: f64, t812: f64, t81599: f64, t87080: f64, t87140: f64, t87155: f64, t98363: f64, t98374: f64, t98380: f64, t98399: f64, t98416: f64, t98420: f64, t98446: f64) -> f64 {
    let t105601 = t1888 * t6646 * t68217 * t232;
    let t105604 = -0.24674011002723396548e-1_f64 * t98363 - 0.57572692339687925277e-1_f64 * t98374 + 0.19190897446562641759e0_f64 * t87080 + 0.57572692339687925277e-1_f64 * t98380 - 0.49348022005446793095e-1_f64 * t105574 - 0.49348022005446793095e-1_f64 * t105578 + 0.14804406601634037928e0_f64 * t105582 + 0.12337005501361698274e-1_f64 * t98399 + 0.49348022005446793095e-1_f64 * t105586 - t81599 + 0.23029076935875170111e0_f64 * t98416 + 0.49348022005446793095e-1_f64 * t87140 - 3.0_f64 * t812 * t25255 * t5612 - 0.23029076935875170111e0_f64 * t98420 + 0.49348022005446793095e-1_f64 * t105596 - 0.49348022005446793095e-1_f64 * t98446 - 0.24674011002723396548e-1_f64 * t105601 + 0.78134368175290755733e-1_f64 * t87155;
    t105604
}
