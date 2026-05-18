//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 993/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk993<F: Float>(t1165: F, t14187: F, t15407: F, t530: F, t322: F, t4162: F, t1532: F, t3451: F, t12816: F, t4267: F, t4282: F, t955: F) -> (F, F, F, F, F) {
    let t16537 = t14187 * t1165 * t530 * t15407;
    let t16539 = t4162 * t322;
    let t16542 = t3451 * t1165 * t1532 * t16539;
    let t16546 = t4282 * t1165 * t4267 * t12816;
    let t16548 = t955 * t322;
    (t16537, t16539, t16542, t16546, t16548)
}
