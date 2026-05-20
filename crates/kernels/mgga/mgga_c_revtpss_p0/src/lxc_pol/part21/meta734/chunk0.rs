//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2582/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2582<F: Float>(t3911: F, t9692: F, t123: F, t1444: F, t3915: F, t9291: F, t2453: F, t9679: F, t138: F, t2438: F, t4077: F, t9302: F, t9674: F) -> (F, F, F, F, F) {
    let t47474 = t3911 * t9692;
    let t47478 = t3915 * t123 * t9291 * t1444;
    let t47480 = t2453 * t9679;
    let t47483 = t47480 * t138 * t2438 * t4077;
    let t47487 = t9674 * t138 * t9302 * t1444;
    (t47474, t47478, t47480, t47483, t47487)
}
