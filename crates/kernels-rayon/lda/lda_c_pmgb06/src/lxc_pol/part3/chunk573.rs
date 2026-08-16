//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 573/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk573(t132: f64, t3055: f64, t1540: f64, t464: f64, t477: f64, t137: f64, t188: f64, t3007: f64, t3009: f64, t3014: f64, t3015: f64, t3019: f64, t3026: f64, t3028: f64, t3037: f64, t3042: f64, t3045: f64, t3049: f64, t3052: f64, t3054: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3056 = t132 * t3055;
    let t3057 = t3056 / 45.0_f64;
    let t3058 = t1540 * t464;
    let t3059 = t3058 * t477;
    let t3060 = t137 * t3059;
    let t3062 = t132 * t3060 / 10.0_f64;
    let t3063 = t3007 - t3009 + t3014 + 4.0_f64 / 3.0_f64 * t3015 * t188 + 4.0_f64 * t3019 + t3026 + 4.0_f64 * t3028 - t3037 + t3042 + t3045 - t3049 - t3052 - t3054 + t3057 - t3062;
    (t3056, t3057, t3058, t3059, t3060, t3062, t3063)
}
