//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 605/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk605(t1583: f64, t405: f64, t1577: f64, t163: f64, t497: f64, t147: f64, t2913: f64, t2939: f64, t525: f64, t740: f64, t146: f64, t164: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3352 = t405 * t1583;
    let t3354 = t405 * t1577;
    let t3357 = 1.0_f64 / t163 / t497;
    let t3358 = t147 * t3357;
    let t3359 = t3358 * t2913;
    let t3362 = t525 * t2939;
    let t3365 = t740 * t147;
    let t3368 = 0.02962962962962963_f64 * t146 * t3365 * t164;
    (t3352, t3354, t3357, t3358, t3359, t3362, t3365, t3368)
}
