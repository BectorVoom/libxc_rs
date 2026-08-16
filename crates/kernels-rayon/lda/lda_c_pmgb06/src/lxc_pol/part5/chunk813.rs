//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 813/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk813(t493: f64, t7671: f64, t6130: f64, t834: f64, t6134: f64, t835: f64, t7633: f64, t7636: f64, t7638: f64, t7642: f64, t7644: f64, t7648: f64, t7650: f64, t7653: f64, t7655: f64, t7658: f64, t7662: f64, t7665: f64, t7669: f64) -> (f64, f64, f64, f64, f64) {
    let t7673 = 2.0_f64 / 15.0_f64 * t493 * t7671;
    let t7674 = t6130 * t834;
    let t7676 = t493 * t7674 / 15.0_f64;
    let t7678 = t6134 * t835 / 15.0_f64;
    let t7679 = t7633 + t7636 + t7638 + t7642 + t7644 - t7648 - t7650 - t7653 - t7655 - t7658 - t7662 + t7665 + t7669 + t7673 + t7676 + t7678;
    (t7673, t7674, t7676, t7678, t7679)
}
