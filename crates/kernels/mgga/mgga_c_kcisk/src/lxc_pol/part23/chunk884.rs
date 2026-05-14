//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 884/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk884<F: Float>(t3969: F, t4396: F, t4369: F, t1308: F, t3973: F, t4402: F, t1580: F, t4384: F, t4397: F, t4387: F, t13917: F, t4392: F, t12829: F, t539: F, t1568: F, t4416: F, sigma0: F) -> (F, F, F, F, F, F, F, F) {
    let t14878 = t4396 * t3969;
    let t14885 = t4369 * sigma0;
    let t14886 = t14885 * t1308;
    let t14915 = t3973 * t4402;
    let t14916 = t1580 * t14915;
    let t14918 = t4397 * t4384;
    let t14921 = t3973 * t4387;
    let t14922 = t1580 * t14921;
    let t14924 = t13917 * t4392;
    let t14925 = t1580 * t14924;
    let t14935 = t539 * t12829;
    let t14940 = t1568 * t4416;
    (t14878, t14886, t14916, t14918, t14922, t14925, t14935, t14940)
}
