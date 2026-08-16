//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 577/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk577(t3010: f64, t3092: f64, t3090: f64, t36: f64, t1437: f64, t5: f64) -> (f64, f64, f64, f64) {
    let t3093 = t3092 * t3010;
    let t3094 = t3090 * t3093;
    let t3095 = t36 * t3094;
    let t3098 = 1.0_f64 / t1437 / t5;
    (t3093, t3094, t3095, t3098)
}
