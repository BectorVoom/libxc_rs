//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 941/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk941<F: Float>(t683: F, t7278: F, t1899: F, t1893: F, t2786: F, t1083: F, t5804: F, t1856: F, t5802: F, t1096: F, t1917: F, t1108: F, t1956: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7279 = t7278 * t683;
    let t7281 = F::new(0.32163958997385070134e2) * t1899 * t7279;
    let t7282 = t2786 * t1893;
    let t7284 = F::new(0.16081979498692535067e2) * t1899 * t7282;
    let t7285 = t1083 * t5804;
    let t7286 = t7285 * t1856;
    let t7288 = F::new(0.51726012919273400301e3) * t5802 * t7286;
    let t7293 = t1096 * t1917;
    let t7296 = t1108 * t1956;
    (t7279, t7281, t7282, t7284, t7285, t7286, t7288, t7293, t7296)
}
