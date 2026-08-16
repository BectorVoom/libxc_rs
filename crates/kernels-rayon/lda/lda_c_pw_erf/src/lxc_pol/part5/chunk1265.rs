//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1265/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1265(t348: f64, t7354: f64, t9763: f64, t34: f64, t6417: f64, t11: f64, t1243: f64, t21196: f64, t1953: f64, t21137: f64, t22277: f64, t503: f64) -> (f64, f64, f64, f64, f64) {
    let t22713 = t9763 * t7354 * t348;
    let t22717 = t6417 * t34;
    let t22722 = t11 * t1243 * t21196;
    let t22725 = t1953 * t1243 * t21137;
    let t22728 = t11 * t503 * t22277;
    (t22713, t22717, t22722, t22725, t22728)
}
