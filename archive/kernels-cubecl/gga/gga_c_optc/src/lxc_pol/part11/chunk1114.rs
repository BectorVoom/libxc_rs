//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1114/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1114<F: Float>(t5301: F, t7878: F, t1179: F, t3137: F, t3186: F, t5407: F, t27515: F, t3244: F, t5355: F, t3169: F, t5344: F, t3138: F, t5280: F) -> (F, F, F, F, F, F) {
    let t46297 = t7878 * t5301;
    let t46298 = t1179 * t46297;
    let t46314 = t3186 * t3137 * t5407;
    let t46390 = t3244 * t27515 * t5355;
    let t46413 = t5344 * t3169;
    let t46452 = t5280 * t3138;
    (t46297, t46298, t46314, t46390, t46413, t46452)
}
