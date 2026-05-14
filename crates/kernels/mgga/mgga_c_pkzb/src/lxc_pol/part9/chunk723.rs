//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 723/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk723<F: Float>(t1673: F, t637: F, t1675: F, t191: F, t1661: F, t545: F, t83: F, t126: F, t5119: F, t1545: F, t546: F, t513: F, t1634: F, t568: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5162 = t1673 * t637;
    let t5165 = 1.0 / t1675 / t191;
    let t5169 = t1661 * t545;
    let t5170 = t83 * t5169;
    let t5171 = 3.0 * t5170;
    let t5175 = t5119 * t126;
    let t5176 = t83 * t5175;
    let t5177 = t1545 * t546;
    let t5178 = 36.0 * t5177;
    let t5179 = t1545 * t513;
    let t5180 = 36.0 * t5179;
    let t5181 = t1634 * t568;
    (t5162, t5165, t5169, t5170, t5171, t5175, t5176, t5177, t5178, t5179, t5180, t5181)
}
