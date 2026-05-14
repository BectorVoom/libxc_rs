//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 904/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk904<F: Float>(t1163: F, t1181: F, t1532: F, t360: F, t4162: F, t1165: F, t1552: F, t372: F, t14575: F, t14187: F, t15407: F, t530: F, t322: F, t3451: F, t12816: F, t4267: F, t4282: F) -> (F, F, F, F, F, F, F) {
    let t16524 = t1163 * t1181 * t1532 * t4162 * t360;
    let t16529 = t1163 * t1165 * t1552 * t4162 * t372;
    let t16533 = t1163 * t1165 * t1532 * t14575;
    let t16537 = t14187 * t1165 * t530 * t15407;
    let t16539 = t4162 * t322;
    let t16542 = t3451 * t1165 * t1532 * t16539;
    let t16546 = t4282 * t1165 * t4267 * t12816;
    (t16524, t16529, t16533, t16537, t16539, t16542, t16546)
}
