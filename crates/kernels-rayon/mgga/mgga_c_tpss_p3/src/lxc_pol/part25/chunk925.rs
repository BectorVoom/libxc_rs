//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 925/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk925(t1253: f64, t3255: f64, t7651: f64, t7653: f64, t7660: f64, t7662: f64, t7669: f64, t7671: f64, t3416: f64, t577: f64, t1286: f64, t1980: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10193 = t3255 * t1253;
    let t10281 = 4.0_f64 * t7651;
    let t10282 = 12.0_f64 * t7653;
    let t10283 = 48.0_f64 * t7660;
    let t10284 = 80.0_f64 * t7662;
    let t10285 = 180.0_f64 * t7669;
    let t10286 = 252.0_f64 * t7671;
    let t10289 = t3416 * t577;
    let t10292 = t1286 * t1980;
    (t10193, t10281, t10282, t10283, t10284, t10285, t10286, t10289, t10292)
}
