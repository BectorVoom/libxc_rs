//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 568/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk568(t1691: f64, t3216: f64, t427: f64, t474: f64, t426: f64, t259: f64, t47: f64, t261: f64, t52: f64, t1686: f64, t19: f64, t436: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3217 = t1691 * t3216;
    let t3227 = t474 * t427;
    let t3228 = t426 * t3227;
    let t3234 = 1.0_f64 / t47 / t259;
    let t3243 = 1.0_f64 / t52 / t261;
    let t3257 = t1686 * t436 * t19;
    (t3217, t3227, t3228, t3234, t3243, t3257)
}
