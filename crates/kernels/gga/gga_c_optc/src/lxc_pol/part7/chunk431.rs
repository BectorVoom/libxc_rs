//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 431/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk431<F: Float>(t2069: F, t696: F, t2073: F, t136: F, t162: F, t2078: F, t159: F, t155: F, t158: F, t652: F) -> (F, F, F, F) {
    let t2171 = t696 * t2069;
    let t2174 = t696 * t2073;
    let t2178 = t2078 * t136 * t162;
    let t2180 = 0.19984346101817798257e0 * t159 * t2178;
    let t2182 = t155 * t158 * t652;
    (t2171, t2174, t2180, t2182)
}
