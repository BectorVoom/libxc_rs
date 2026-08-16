//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2918/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2918(t3911: f64, t9692: f64, t123: f64, t1444: f64, t3915: f64, t9291: f64, t2453: f64, t9679: f64, t138: f64, t2438: f64, t4077: f64, t9302: f64, t9674: f64) -> (f64, f64, f64, f64, f64) {
    let t47474 = t3911 * t9692;
    let t47478 = t3915 * t123 * t9291 * t1444;
    let t47480 = t2453 * t9679;
    let t47483 = t47480 * t138 * t2438 * t4077;
    let t47487 = t9674 * t138 * t9302 * t1444;
    (t47474, t47478, t47480, t47483, t47487)
}
