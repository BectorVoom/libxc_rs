//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 718/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk718<F: Float>(t1: F, t6855: F, t1274: F, t6893: F, t140: F, t6916: F, t1261: F, t6956: F, t1281: F, t2105: F, t6917: F, t2029: F, t3500: F, t1278: F, t7061: F, t1291: F, t7073: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9747 = t6855 * t1;
    let t9769 = t6893 * t1274;
    let t9771 = t6916 * t140;
    let t9782 = t6956 * t1261;
    let t9794 = t1281 * t2105;
    let t9839 = t6917 * t140;
    let t9896 = t3500 * t2029;
    let t9913 = t7061 * t1278;
    let t9915 = t7073 * t1291;
    (t9747, t9769, t9771, t9782, t9794, t9839, t9896, t9913, t9915)
}
