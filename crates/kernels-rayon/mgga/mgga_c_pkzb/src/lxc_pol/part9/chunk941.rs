//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 941/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk941(t683: f64, t7278: f64, t1899: f64, t1893: f64, t2786: f64, t1083: f64, t5804: f64, t1856: f64, t5802: f64, t1096: f64, t1917: f64, t1108: f64, t1956: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7279 = t7278 * t683;
    let t7281 = 0.32163958997385070134e2_f64 * t1899 * t7279;
    let t7282 = t2786 * t1893;
    let t7284 = 0.16081979498692535067e2_f64 * t1899 * t7282;
    let t7285 = t1083 * t5804;
    let t7286 = t7285 * t1856;
    let t7288 = 0.51726012919273400301e3_f64 * t5802 * t7286;
    let t7293 = t1096 * t1917;
    let t7296 = t1108 * t1956;
    (t7279, t7281, t7282, t7284, t7285, t7286, t7288, t7293, t7296)
}
