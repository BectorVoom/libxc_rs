//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 552/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk552(t144: f64, t3031: f64, t1600: f64, t511: f64, t1547: f64, t478: f64, t132: f64, t134: f64, t138: f64, t2897: f64, t455: f64, t947: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3032 = t144 * t3031;
    let t3038 = t511 * t1600;
    let t3055 = t1547 * t478;
    let t3056 = t132 * t3055;
    let t3080 = t138 * t2897 * t134;
    let t3081 = 0.005877407407407408_f64 * t3080;
    let t3082 = t947 * t455;
    (t3032, t3038, t3055, t3056, t3080, t3081, t3082)
}
