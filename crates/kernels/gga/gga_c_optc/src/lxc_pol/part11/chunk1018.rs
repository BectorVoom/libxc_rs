//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1018/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1018<F: Float>(t1133: F, t45718: F, t46297: F, t12489: F, t4369: F, t12121: F, t4310: F, t45693: F, t3116: F, t35165: F, t5324: F, t5311: F, t8446: F, t1111: F, t530: F, t5318: F) -> (F, F, F, F, F, F, F, F) {
    let t46832 = t1133 * t45718;
    let t46851 = t1133 * t46297;
    let t46853 = t4369 * t12489;
    let t46886 = t4310 * t12121;
    let t46902 = t1133 * t45693;
    let t46923 = t3116 * t35165 * t5324;
    let t46945 = t8446 * t5311;
    let t47001 = t1111 * t530 * t5318;
    (t46832, t46851, t46853, t46886, t46902, t46923, t46945, t47001)
}
