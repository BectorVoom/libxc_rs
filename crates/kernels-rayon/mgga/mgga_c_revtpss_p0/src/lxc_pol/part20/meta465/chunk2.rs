//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1778/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1778(t138: f64, t2438: f64, t4077: f64, t47480: f64, t1444: f64, t9302: f64, t9674: f64, t10009: f64, t1364: f64, t786: f64, t3899: f64, t4078: f64, t689: f64) -> (f64, f64, f64, f64) {
    let t47483 = t47480 * t138 * t2438 * t4077;
    let t47487 = t9674 * t138 * t9302 * t1444;
    let t47490 = t786 * t10009 * t1364;
    let t47493 = t689 * t3899 * t4078;
    (t47483, t47487, t47490, t47493)
}
