//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1303/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1303(t405: f64, t6885: f64, t4913: f64, t6888: f64, t4641: f64, t6816: f64, t350: f64, t6805: f64, t16073: f64, t36: f64, t506: f64, t6824: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17127 = t405 * t6885;
    let t17129 = t4913 * t6888;
    let t17131 = t4641 * t6816;
    let t17133 = t350 * t6805;
    let t17136 = t36 * t506 * t16073;
    let t17138 = t350 * t6824;
    (t17127, t17129, t17131, t17133, t17136, t17138)
}
