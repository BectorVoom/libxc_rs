//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1112/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1112<F: Float>(t3107: F, t45811: F, t27351: F, t3234: F, t5355: F, t1162: F, t5285: F, t7274: F, t5289: F, t3138: F, t5417: F, t44090: F) -> (F, F, F, F, F, F) {
    let t45812 = t45811 * t3107;
    let t45885 = t3234 * t27351 * t5355;
    let t45954 = t1162 * t7274 * t5285;
    let t45968 = t1162 * t7274 * t5289;
    let t46007 = t5417 * t3138;
    let t46014 = t44090 * t3107;
    (t45812, t45885, t45954, t45968, t46007, t46014)
}
