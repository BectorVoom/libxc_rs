//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1124/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1124(t16877: f64, t6731: f64, t831: f64, t16880: f64, t16884: f64, t13177: f64, t16920: f64, t16922: f64, t16925: f64, t16927: f64, t16936: f64, t16962: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20501 = t16877 / 15.0_f64;
    let t20503 = t831 * t6731 / 5.0_f64;
    let t20504 = 2.0_f64 / 45.0_f64 * t16880;
    let t20505 = 2.0_f64 / 15.0_f64 * t16884;
    let t20506 = 8.0_f64 / 405.0_f64 * t13177;
    let t20507 = 2.0_f64 / 45.0_f64 * t16920;
    let t20508 = 4.0_f64 / 45.0_f64 * t16922;
    let t20509 = 2.0_f64 / 45.0_f64 * t16925;
    let t20510 = 2.0_f64 / 45.0_f64 * t16927;
    let t20511 = t16936 / 15.0_f64;
    let t20512 = 2.0_f64 / 45.0_f64 * t16962;
    (t20501, t20503, t20504, t20505, t20506, t20507, t20508, t20509, t20510, t20511, t20512)
}
