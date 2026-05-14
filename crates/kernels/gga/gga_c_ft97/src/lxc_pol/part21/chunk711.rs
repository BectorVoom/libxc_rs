//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 711/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk711<F: Float>(t167: F, t17076: F, t2185: F, t4724: F, t558: F, t2179: F, t574: F, t609: F, t9439: F, t144: F, t1882: F, t4730: F, t4458: F, t569: F, t616: F, t12752: F, t17041: F, t17045: F, t17049: F, t17053: F, t17057: F, t17060: F, t17063: F, t17068: F, t17073: F, t1901: F, t446: F) -> (F, F, F) {
    let t17078 = t2185 * t167 * t17076;
    let t17081 = t4724 * t558;
    let t17083 = t574 * t2179 * t17081;
    let t17086 = t4724 * t609;
    let t17087 = t9439 * t17086;
    let t17088 = t144 * t17087;
    let t17091 = t1882 * t4730;
    let t17095 = t569 * t616 * t4458;
    let t17098 = 2.0 / 27.0 * t1901 * t17041 + 2.0 / 27.0 * t1901 * t17045 + 4.0 / 9.0 * t1901 * t17049 + 2.0 / 9.0 * t1901 * t17053 - 2.0 / 27.0 * t1901 * t17057 + t17060 / 9.0 - 2.0 / 3.0 * t446 * t17063 - 2.0 * t446 * t17068 - 2.0 / 3.0 * t446 * t17073 + 4.0 / 3.0 * t446 * t17078 - 2.0 / 3.0 * t446 * t17083 - 2.0 * t446 * t17088 - 2.0 / 9.0 * t17091 + 8.0 / 27.0 * t12752 + 2.0 / 9.0 * t446 * t17095;
    (t17081, t17086, t17098)
}
