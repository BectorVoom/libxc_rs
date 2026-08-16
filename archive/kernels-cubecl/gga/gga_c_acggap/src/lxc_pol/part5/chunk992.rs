//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 992/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk992<F: Float>(t1181: F, t16507: F, t3391: F, t4417: F, t1163: F, t1532: F, t360: F, t4162: F, t1165: F, t1552: F, t372: F, t14575: F) -> (F, F, F, F) {
    let t16510 = t3391 * t1181 * t4417 * t16507;
    let t16524 = t1163 * t1181 * t1532 * t4162 * t360;
    let t16529 = t1163 * t1165 * t1552 * t4162 * t372;
    let t16533 = t1163 * t1165 * t1532 * t14575;
    (t16510, t16524, t16529, t16533)
}
