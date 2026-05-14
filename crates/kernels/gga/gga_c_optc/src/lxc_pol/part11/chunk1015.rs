//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1015/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1015<F: Float>(t5328: F, t8459: F, t115: F, t5274: F, t911: F, t3241: F, t1027: F, t8915: F, t1170: F, t5388: F, t7878: F, t5301: F, t1179: F, t3137: F, t3186: F, t5407: F) -> (F, F, F, F, F, F, F) {
    let t46152 = t8459 * t5328;
    let t46171 = t5274 * t911 * t115;
    let t46172 = t3241 * t46171;
    let t46193 = t8915 * t1027;
    let t46242 = t1170 * t7878 * t5388;
    let t46297 = t7878 * t5301;
    let t46298 = t1179 * t46297;
    let t46314 = t3186 * t3137 * t5407;
    (t46152, t46172, t46193, t46242, t46297, t46298, t46314)
}
