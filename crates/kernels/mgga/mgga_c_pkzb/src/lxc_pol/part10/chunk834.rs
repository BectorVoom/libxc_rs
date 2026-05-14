//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 834/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk834<F: Float>(t1545: F, t513: F, t1542: F, t546: F, t1548: F, t1816: F, t639: F, t1692: F, t192: F, t1705: F, t575: F) -> (F, F, F, F, F, F, F) {
    let t5179 = t1545 * t513;
    let t5186 = 60.0 * t1542 * t546;
    let t5187 = t1548 * t513;
    let t5189 = t1542 * t513;
    let t5191 = t1816 * t639;
    let t5196 = t192 * t1692;
    let t5221 = t575 * t1705;
    (t5179, t5186, t5187, t5189, t5191, t5196, t5221)
}
