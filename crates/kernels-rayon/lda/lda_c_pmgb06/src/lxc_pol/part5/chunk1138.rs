//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1138/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1138(t1447: f64, t7660: f64, t2465: f64, t493: f64, t5312: f64, t17372: f64, t17374: f64, t1972: f64, t6120: f64, t17376: f64, t2002: f64, t6556: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20662 = t1447 * t7660;
    let t20663 = 4.0_f64 / 27.0_f64 * t20662;
    let t20666 = t493 * t5312 * t2465 / 15.0_f64;
    let t20667 = 2.0_f64 / 45.0_f64 * t17372;
    let t20668 = t17374 / 45.0_f64;
    let t20670 = 2.0_f64 / 5.0_f64 * t1972 * t6120;
    let t20671 = 2.0_f64 / 27.0_f64 * t17376;
    let t20673 = t2002 * t6556 / 5.0_f64;
    (t20663, t20666, t20667, t20668, t20670, t20671, t20673)
}
