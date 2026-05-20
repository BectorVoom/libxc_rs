//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2458/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2458<F: Float>(t39494: F, t3964: F, t4096: F, t40270: F, t4089: F, t3911: F, t9692: F, t123: F, t1444: F, t3915: F, t9291: F, t2453: F, t9679: F) -> (F, F, F, F, F) {
    let t47454 = F::cast_from(0.20561456923286030469e-1_f64) * t3964 * t4096 * t39494;
    let t47455 = t40270 * t4089;
    let t47474 = t3911 * t9692;
    let t47478 = t3915 * t123 * t9291 * t1444;
    let t47480 = t2453 * t9679;
    (t47454, t47455, t47474, t47478, t47480)
}
