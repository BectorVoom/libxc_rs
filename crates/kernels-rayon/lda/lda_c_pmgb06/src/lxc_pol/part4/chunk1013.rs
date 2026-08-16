//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1013/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1013(t1179: f64, t4068: f64, t573: f64, t580: f64, t1147: f64, t206: f64, t208: f64, t31: f64, t99: f64, t213: f64, t398: f64, t4075: f64) -> (f64, f64, f64, f64) {
    let t9457 = t573 * t1179 * t4068;
    let t9461 = 0.006061752703703704_f64 * t580 * t1179 * t4068;
    let t9467 = 0.0002763148940771605_f64 * t206 * t1147 * t99 * t31 * t208;
    let t9470 = t398 * t4075 * t208 * t213;
    (t9457, t9461, t9467, t9470)
}
