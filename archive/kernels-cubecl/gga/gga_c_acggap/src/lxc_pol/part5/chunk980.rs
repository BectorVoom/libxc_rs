//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 980/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk980<F: Float>(t4355: F, t997: F, t13463: F, t171: F, t1008: F, t4528: F, t3372: F, t5129: F, t1163: F, t1165: F, t1532: F, t16020: F) -> (F, F, F, F, F) {
    let t16057 = t997 * t4355;
    let t16059 = t13463 * t171;
    let t16072 = t1008 * t4528;
    let t16083 = t3372 * t5129;
    let t16110 = t1163 * t1165 * t1532 * t16020;
    (t16057, t16059, t16072, t16083, t16110)
}
