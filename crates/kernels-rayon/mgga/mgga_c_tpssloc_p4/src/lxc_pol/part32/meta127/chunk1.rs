//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 731/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk731(t1094: f64, t1098: f64, t1097: f64, t419: f64, t409: f64, t407: f64, t410: f64, t3236: f64, t281: f64, t2820: f64, t415: f64, t1114: f64, t699: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3259 = t1094 * t1098;
    let t3262 = t1097 * t419;
    let t3263 = 1.0_f64 / t3262;
    let t3264 = t409 * t3263;
    let t3270 = 1.0_f64 / t410 / t407;
    let t3274 = 4.0_f64 / 9.0_f64 * t3236;
    let t3282 = 0.39862222222222222223e0_f64 * t3236;
    let t3287 = 1.0_f64/f64::sqrt(t407);
    let t3293 = t281 * t2820 * t415;
    let t3294 = 0.13692777777777777778e0_f64 * t3293;
    let t3295 = t699 * t1114;
    (t3259, t3263, t3264, t3270, t3274, t3282, t3287, t3293, t3294, t3295)
}
