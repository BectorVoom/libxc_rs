//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 571/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk571(t144: f64, t3031: f64, t1594: f64, t477: f64, t137: f64, t132: f64, t1600: f64, t511: f64, t1602: f64, t166: f64, t161: f64, t1603: f64, t489: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3032 = t144 * t3031;
    let t3033 = t1594 * t477;
    let t3034 = t3032 * t3033;
    let t3035 = t137 * t3034;
    let t3037 = t132 * t3035 / 5.0_f64;
    let t3038 = t511 * t1600;
    let t3039 = t3038 * t1602;
    let t3040 = t166 * t3039;
    let t3042 = t161 * t3040 / 5.0_f64;
    let t3043 = t489 * t1603;
    (t3032, t3033, t3034, t3035, t3037, t3038, t3039, t3040, t3042, t3043)
}
