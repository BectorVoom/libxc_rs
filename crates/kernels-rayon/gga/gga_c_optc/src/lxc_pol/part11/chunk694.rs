//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 694/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk694(t133: f64, t155: f64, t6991: f64, t108: f64, t2156: f64, t110: f64, t146: f64, t2157: f64, t652: f64, t2078: f64, t693: f64, t2002: f64, t671: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6993 = t155 * t6991 * t133;
    let t7000 = t2156 * t108;
    let t7002 = t146 * t7000 * t110;
    let t7018 = t155 * t2157 * t652;
    let t7022 = t155 * t693 * t2078;
    let t7030 = t146 * t671 * t2002;
    (t6993, t7000, t7002, t7018, t7022, t7030)
}
