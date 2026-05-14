//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 930/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk930<F: Float>(t12084: F, t12105: F, t12126: F, t12147: F, t576: F, t3848: F, t699: F, t1096: F, t11043: F, t3828: F, t883: F, t972: F, t1125: F, t9375: F, t3449: F, t3565: F) -> (F, F, F, F, F, F, F, F) {
    let t12149 = t12084 + t12105 + t12126 + t12147;
    let t12150 = t576 * t12149;
    let t12151 = t699 * t3848;
    let t12152 = t11043 * t1096;
    let t12153 = t3828 * t883;
    let t12154 = t12153 * t972;
    let t12155 = t9375 * t1125;
    let t12156 = t3565 * t3449;
    (t12149, t12150, t12151, t12152, t12153, t12154, t12155, t12156)
}
