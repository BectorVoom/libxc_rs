//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 490/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk490<F: Float>(t3399: F, t40: F, t1264: F, t138: F, t1: F, t2060: F, t123: F, t1256: F, t1278: F, t654: F, t130: F, t635: F, t140: F, t2086: F, t6: F, t1281: F, t669: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3400 = t40 * t3399;
    let t3406 = t1264 * t138;
    let t3411 = t2060 * t1;
    let t3412 = t123 * t1256;
    let t3437 = t654 * t1278;
    let t3439 = t130 * t635;
    let t3440 = t2086 * t140;
    let t3441 = t6 * t1256;
    let t3454 = t1281 * t669;
    (t3400, t3406, t3411, t3412, t3437, t3439, t3440, t3441, t3454)
}
