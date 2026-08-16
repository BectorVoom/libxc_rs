//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 714/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk714(t1525: f64, t4655: f64, t1830: f64, t1074: f64, t1858: f64, t36: f64, t1069: f64, t453: f64, t1: f64, t1531: f64, t332: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4656 = t1525 * t4655;
    let t4657 = t1830 * t4656;
    let t4659 = t1858 * t1074;
    let t4660 = t1525 * t4659;
    let t4661 = t36 * t4660;
    let t4663 = t1858 * t1069;
    let t4664 = t453 * t4663;
    let t4665 = t36 * t4664;
    let t4667 = t1531 * t1;
    let t4668 = t4667 * t332;
    let t4669 = t453 * t4668;
    let t4670 = t1830 * t4669;
    (t4656, t4657, t4659, t4660, t4661, t4663, t4664, t4665, t4667, t4668, t4669, t4670)
}
