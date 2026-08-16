//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1778/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1778<F: Float>(t138: F, t2438: F, t4077: F, t47480: F, t1444: F, t9302: F, t9674: F, t10009: F, t1364: F, t786: F, t3899: F, t4078: F, t689: F) -> (F, F, F, F) {
    let t47483 = t47480 * t138 * t2438 * t4077;
    let t47487 = t9674 * t138 * t9302 * t1444;
    let t47490 = t786 * t10009 * t1364;
    let t47493 = t689 * t3899 * t4078;
    (t47483, t47487, t47490, t47493)
}
