//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 279/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk279(t248: f64, t283: f64, t619: f64, t636: f64, t640: f64, t645: f64, t688: f64, t695: f64, t700: f64, t897: f64, t898: f64) -> f64 {
    let t902 = t619 + t636 - t640 - t645 + t248 * t898 + t688 + 0.0197516734986138_f64 * t897 * t283 - t695 - t700;
    t902
}
