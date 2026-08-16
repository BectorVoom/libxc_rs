//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 940/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk940(t3505: f64, t3513: f64, t3515: f64, t3517: f64, t5801: f64, t5808: f64, t5813: f64, t63: f64, t7012: f64, t7013: f64, t7017: f64, t7018: f64, t7039: f64) -> f64 {
    let t7041 = t7012 + t5801 + t7013 + t5808 - 1.95872_f64 * t5813 - t7017 - 1.46904_f64 * t63 * t7018 - t3505 + t3513 - t3515 - t3517 + t7039;
    t7041
}
