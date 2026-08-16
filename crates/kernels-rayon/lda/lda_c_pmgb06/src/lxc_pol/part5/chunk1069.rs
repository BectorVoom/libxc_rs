//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1069/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1069(t1830: f64, t19475: f64, t3090: f64, t1531: f64, t7290: f64, t332: f64, t36: f64, t453: f64, t1525: f64, t19490: f64, t12396: f64, t12406: f64, t19618: f64) -> (f64, f64, f64, f64, f64) {
    let t19799 = t1830 * t3090 * t19475;
    let t19801 = t1531 * t7290;
    let t19802 = t19801 * t332;
    let t19804 = t36 * t453 * t19802;
    let t19807 = t36 * t1525 * t19490;
    let t19811 = t12396 * t12406 * t19618;
    (t19799, t19802, t19804, t19807, t19811)
}
