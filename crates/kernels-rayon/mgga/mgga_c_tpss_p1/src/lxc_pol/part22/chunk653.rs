//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 653/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk653(t3025: f64, t441: f64, t1102: f64, t140: f64, t1098: f64, t1014: f64, t390: f64, t2840: f64, t1985: f64, t926: f64, t1100: f64, t2845: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3027 = t441 * t3025 / 432.0_f64;
    let t3028 = t140 * t1102;
    let t3029 = t1098 * t3028;
    let t3032 = 1.0_f64 / t390 / t1014;
    let t3033 = t3032 * t2840;
    let t3034 = t3033 * t1985;
    let t3035 = t926 * t3034;
    let t3038 = t1100 * t2845;
    (t3027, t3028, t3029, t3032, t3034, t3035, t3038)
}
