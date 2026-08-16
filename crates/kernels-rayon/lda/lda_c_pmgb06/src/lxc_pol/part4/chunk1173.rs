//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1173/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1173(t30: f64, t32: f64, t8083: f64, t1438: f64, t79: f64, t1525: f64, t1531: f64, t453: f64, t350: f64, t6221: f64, t1830: f64, t1863: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15426 = t30 * t32 * t8083;
    let t15427 = t1438 * t79;
    let t15429 = t15426 * t1525 * t15427;
    let t15431 = t1531 * t79;
    let t15433 = t15426 * t453 * t15431;
    let t15435 = t350 * t6221;
    let t15438 = t1830 * t453 * t1863;
    (t15426, t15427, t15429, t15431, t15433, t15435, t15438)
}
