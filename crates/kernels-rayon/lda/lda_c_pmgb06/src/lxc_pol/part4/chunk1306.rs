//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1306/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1306(t1083: f64, t6759: f64, t36: f64, t506: f64, t350: f64, t6802: f64, t4641: f64, t6808: f64, t16905: f64, t2909: f64, t16910: f64, t9507: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17160 = t6759 * t1083;
    let t17162 = t36 * t506 * t17160;
    let t17164 = t350 * t6802;
    let t17166 = t4641 * t6808;
    let t17169 = t36 * t2909 * t16905;
    let t17172 = t36 * t9507 * t16910;
    (t17160, t17162, t17164, t17166, t17169, t17172)
}
