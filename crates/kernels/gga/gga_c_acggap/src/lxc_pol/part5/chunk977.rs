//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 977/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk977<F: Float>(t1165: F, t1532: F, t15407: F, t3456: F, t1487: F, t435: F, t3375: F, t4987: F, t1163: F, t1586: F, t4210: F, t14575: F, t540: F) -> (F, F, F, F, F) {
    let t15982 = t3456 * t1165 * t1532 * t15407;
    let t15995 = t435 * t1487;
    let t16008 = t3375 * t4987;
    let t16013 = t1163 * t1165 * t1586 * t4210;
    let t16017 = t1163 * t1165 * t540 * t14575;
    (t15982, t15995, t16008, t16013, t16017)
}
