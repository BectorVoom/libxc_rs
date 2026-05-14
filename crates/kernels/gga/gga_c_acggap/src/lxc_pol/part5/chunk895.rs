//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 895/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk895<F: Float>(t1008: F, t4528: F, t3372: F, t5129: F, t1163: F, t1165: F, t1532: F, t16020: F, t3379: F, t5272: F, t3375: F, t5133: F, t4987: F, t13502: F, t542: F, t1569: F, t3237: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16072 = t1008 * t4528;
    let t16083 = t3372 * t5129;
    let t16110 = t1163 * t1165 * t1532 * t16020;
    let t16117 = t3379 * t5272;
    let t16123 = t3375 * t5129;
    let t16125 = t3375 * t5133;
    let t16127 = t3372 * t4987;
    let t16141 = t13502 * t542;
    let t16143 = t3237 * t1569;
    (t16072, t16083, t16110, t16117, t16123, t16125, t16127, t16141, t16143)
}
