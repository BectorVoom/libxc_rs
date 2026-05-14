//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 843/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk843<F: Float>(t322: F, t7932: F, t609: F, t848: F, t464: F, t2122: F, t323: F, t851: F, t14575: F, t7942: F, t7884: F, t7941: F, t15758: F, t2132: F, t7896: F, t7997: F) -> (F, F, F, F, F, F, F, F) {
    let t32004 = t7932 * t322;
    let t32029 = t848 * t609;
    let t32030 = t32029 * t464;
    let t32033 = t851 * t2122 * t323;
    let t32036 = t7942 * t7932 * t14575;
    let t32041 = t7884 * t7941;
    let t32043 = t32041 * t7932 * t15758;
    let t32048 = 0.52041769129231196772e1 * t7896 * t2132 * t7997 * t322;
    (t32004, t32029, t32030, t32033, t32036, t32041, t32043, t32048)
}
