//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 242/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk242(t696: f64, t698: f64, t248: f64, t283: f64, t619: f64, t636: f64, t640: f64, t645: f64, t653: f64, t654: f64, t688: f64, t695: f64) -> (f64, f64) {
    let t700 = 0.5848223622634646_f64 * t696 * t698;
    let t701 = t619 + t636 + t640 - t645 + t248 * t654 + t688 + 0.0197516734986138_f64 * t653 * t283 - t695 - t700;
    (t700, t701)
}
