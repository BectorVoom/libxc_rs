//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1146/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1146(t486: f64, t4937: f64, t161: f64, t4802: f64, t489: f64, t4754: f64, t479: f64, t132: f64, t137: f64, t2064: f64, t3058: f64, t166: f64, t2093: f64, t3382: f64) -> (f64, f64, f64, f64, f64) {
    let t13684 = t486 * t4937 / 10.0_f64;
    let t13686 = t161 * t489 * t4802;
    let t13687 = 2.0_f64 / 15.0_f64 * t13686;
    let t13689 = t4754 * t479 / 10.0_f64;
    let t13693 = t132 * t137 * t3058 * t2064 / 10.0_f64;
    let t13697 = t161 * t166 * t2093 * t3382 / 30.0_f64;
    (t13684, t13687, t13689, t13693, t13697)
}
