//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 987/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk987<F: Float>(t16884: F, t13177: F, t16920: F, t16922: F, t16925: F, t16927: F, t16936: F, t16962: F, t13140: F, t20501: F, t20503: F, t20504: F, t16964: F, t16966: F, t16968: F, t16970: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t20505 = 2.0 / 15.0 * t16884;
    let t20506 = 8.0 / 405.0 * t13177;
    let t20507 = 2.0 / 45.0 * t16920;
    let t20508 = 4.0 / 45.0 * t16922;
    let t20509 = 2.0 / 45.0 * t16925;
    let t20510 = 2.0 / 45.0 * t16927;
    let t20511 = t16936 / 15.0;
    let t20512 = 2.0 / 45.0 * t16962;
    let t20513 = -t20501 - t20503 + t20504 - t20505 - t13140 + t20506 + t20507 + t20508 + t20509 + t20510 - t20511 + t20512;
    let t20515 = 2.0 / 45.0 * t16964;
    let t20516 = 2.0 / 45.0 * t16966;
    let t20517 = 2.0 / 27.0 * t16968;
    let t20518 = 2.0 / 27.0 * t16970;
    (t20505, t20506, t20507, t20508, t20509, t20510, t20511, t20512, t20513, t20515, t20516, t20517, t20518)
}
