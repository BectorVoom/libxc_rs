//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 871/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk871<F: Float>(t10075: F, t3187: F, t406: F, t3899: F, t6475: F, t2380: F, t2029: F, t3880: F, t3207: F, t3913: F, t6411: F, t2381: F, t2370: F, t3199: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10076 = t10075 * t3187;
    let t10077 = t406 * t10076;
    let t10080 = t6475 * t3899;
    let t10081 = t2380 * t10080;
    let t10083 = t3880 * t2029;
    let t10084 = t10083 * t3207;
    let t10085 = t406 * t10084;
    let t10088 = t3913 * t6411;
    let t10089 = t2381 * t10088;
    let t10092 = t2370 * t3199;
    (t10076, t10077, t10080, t10081, t10083, t10084, t10085, t10088, t10089, t10092)
}
