//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 892/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk892<F: Float>(t2787: F, t5771: F, t2783: F, t683: F, t1855: F, t1084: F, t1893: F, t1856: F, t2786: F, t5776: F, t1901: F, t2782: F, t1899: F, t1083: F, t5804: F, t5802: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7268 = 0.32163958997385070134e2 * t5771 * t2787;
    let t7269 = t2783 * t683;
    let t7271 = 4.0 * t1855 * t7269;
    let t7272 = t1084 * t1893;
    let t7274 = 2.0 * t1855 * t7272;
    let t7275 = t2786 * t1856;
    let t7277 = 0.96491876992155210402e2 * t5776 * t7275;
    let t7278 = t2782 * t1901;
    let t7279 = t7278 * t683;
    let t7281 = 0.32163958997385070134e2 * t1899 * t7279;
    let t7282 = t2786 * t1893;
    let t7284 = 0.16081979498692535067e2 * t1899 * t7282;
    let t7285 = t1083 * t5804;
    let t7286 = t7285 * t1856;
    let t7288 = 0.51726012919273400301e3 * t5802 * t7286;
    (t7268, t7269, t7271, t7272, t7274, t7275, t7277, t7278, t7279, t7281, t7282, t7284, t7285, t7286, t7288)
}
