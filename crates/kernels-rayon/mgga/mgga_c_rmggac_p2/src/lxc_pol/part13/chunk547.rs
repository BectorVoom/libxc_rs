//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 547/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk547(t236: f64, t4545: f64, t1971: f64, t7365: f64, t4510: f64, t1970: f64, t352: f64, t498: f64, t515: f64, t7231: f64, t3351: f64, t4048: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7366 = t236 * t4545;
    let t7367 = t1971 * t7366;
    let t7368 = t7365 * t7367;
    let t7370 = t236 * t4510;
    let t7371 = t1971 * t7370;
    let t7372 = t1970 * t7371;
    let t7374 = t352 * t498;
    let t7375 = t515 * t7374;
    let t7376 = t7231 * t7375;
    let t7377 = t3351 * t7376;
    let t7379 = t515 * t4048;
    (t7367, t7368, t7371, t7372, t7376, t7377, t7379)
}
