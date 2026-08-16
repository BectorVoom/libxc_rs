//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1253/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1253(t1138: f64, t2817: f64, t2820: f64, t780: f64, t153: f64, t474: f64, t5718: f64, t168: f64, t2782: f64, t861: f64, t1125: f64, t1891: f64) -> (f64, f64, f64, f64) {
    let t14911 = t2817 * t780 * t1138 * t2820;
    let t14921 = t153 * t474 * t5718;
    let t14925 = t168 * t2782 * t861;
    let t14932 = t153 * t1125 * t1891;
    (t14911, t14921, t14925, t14932)
}
