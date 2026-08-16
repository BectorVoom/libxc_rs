//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 596/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk596(t3239: f64, t493: f64, t2938: f64, t498: f64, t496: f64, t1414: f64, t164: f64) -> (f64, f64, f64, f64, f64) {
    let t3241 = t493 * t3239 / 9.0_f64;
    let t3242 = t498 * t2938;
    let t3243 = t496 * t3242;
    let t3245 = t493 * t3243 / 45.0_f64;
    let t3247 = 1.0_f64 / t164 / t1414;
    (t3241, t3242, t3243, t3245, t3247)
}
