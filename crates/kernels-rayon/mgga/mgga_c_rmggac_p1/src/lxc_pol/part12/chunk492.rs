//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 492/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk492(t60: f64, t3998: f64, t525: f64, t50: f64, t921: f64, t284: f64, t814: f64, t1403: f64, t1406: f64, t154: f64, t62: f64, t922: f64, t925: f64, zeta_threshold: f64) -> (f64, f64) {
    let t61 = t60 <= zeta_threshold;
    let t5339 = t3998 * t525;
    let t5342 = t921 * t50;
    let t5343 = t814 * t284;
    let t5353 = piecewise3(t61, 0.0_f64, -8.0_f64 / 27.0_f64 * t5339 * t922 - 16.0_f64 / 9.0_f64 * t5342 * t5343 + 4.0_f64 / 9.0_f64 * t1403 * t925 - 8.0_f64 / 3.0_f64 * t62 * t814 + 8.0_f64 * t1406 * t154);
    (t5343, t5353)
}
