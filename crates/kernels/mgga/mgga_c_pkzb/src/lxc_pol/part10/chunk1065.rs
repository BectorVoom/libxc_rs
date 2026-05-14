//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1065/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1065<F: Float>(t1855: F, t9219: F, t3554: F, t683: F, t5776: F, t3551: F, t1901: F, t3550: F, t1899: F, t2782: F, t2786: F, t3524: F, t5804: F, t5802: F, t237: F, t3586: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9221 = 4.0 * t1855 * t9219;
    let t9222 = t3554 * t683;
    let t9224 = 0.96491876992155210402e2 * t5776 * t9222;
    let t9225 = t3551 * t683;
    let t9227 = 2.0 * t1855 * t9225;
    let t9228 = t3550 * t1901;
    let t9229 = t9228 * t683;
    let t9231 = 0.16081979498692535067e2 * t1899 * t9229;
    let t9232 = t2786 * t2782;
    let t9234 = 0.32163958997385070134e2 * t1899 * t9232;
    let t9235 = t3524 * t5804;
    let t9236 = t9235 * t683;
    let t9238 = 0.51726012919273400301e3 * t5802 * t9236;
    let t9242 = t237 * t3586;
    (t9221, t9222, t9224, t9225, t9227, t9228, t9229, t9231, t9232, t9234, t9235, t9236, t9238, t9242)
}
