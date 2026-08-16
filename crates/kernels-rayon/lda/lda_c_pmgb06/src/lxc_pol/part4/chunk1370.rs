//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1370/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1370(t1436: f64, t1439: f64, t17579: f64, t79: f64, t13726: f64, t806: f64, t2007: f64, t5220: f64, t2012: f64, t5210: f64, t801: f64, t1415: f64, t15845: f64, t496: f64) -> (f64, f64, f64, f64, f64) {
    let t17990 = 8.0_f64 / 27.0_f64 * t17579 * t1436 * t1439 * t79;
    let t17991 = t13726 * t806;
    let t17992 = 8.0_f64 / 135.0_f64 * t17991;
    let t17993 = t5220 * t2007;
    let t17994 = 8.0_f64 / 135.0_f64 * t17993;
    let t17996 = t801 * t5210 * t2012;
    let t17997 = 4.0_f64 / 27.0_f64 * t17996;
    let t18001 = 16.0_f64 / 45.0_f64 * t15845 * t496 * t1415 * t79;
    (t17990, t17992, t17994, t17997, t18001)
}
