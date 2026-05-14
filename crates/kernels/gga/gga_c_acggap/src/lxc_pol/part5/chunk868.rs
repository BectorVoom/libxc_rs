//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 868/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk868<F: Float>(t2712: F, t2716: F, t2736: F, t2978: F, t2982: F, t2988: F, t2805: F, t2811: F, t2994: F, t1364: F, t228: F, t1372: F, t1357: F, t2604: F, t1381: F, t2632: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t14718 = 0.2137e0 * t2712;
    let t14719 = 0.34367190188705947438e1 * t2716;
    let t14720 = 0.4274e0 * t2736;
    let t14725 = 480.0 * t2978;
    let t14726 = 0.23392894490538584828e1 * t2982;
    let t14729 = 0.14035736694323150897e2 * t2988;
    let t14731 = 0.2069040516770936012e4 * t2805;
    let t14732 = 0.3859675079686208416e3 * t2811;
    let t14734 = 240.0 * t2994;
    let t14810 = 32.0 * t1364 * t228;
    let t14837 = 32.0 * t1372 * t228;
    let t14852 = t1357 * t2604;
    let t14854 = t1381 * t2632;
    (t14718, t14719, t14720, t14725, t14726, t14729, t14731, t14732, t14734, t14810, t14837, t14852, t14854)
}
