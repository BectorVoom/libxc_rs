//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 802/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk802(t2633: f64, t2642: f64, t2651: f64, t2678: f64, t3617: f64, t3640: f64, t3643: f64, t3645: f64, t3650: f64, t4222: f64, t4352: f64, t4475: f64, t493: f64) -> f64 {
    let t4481 = -t4222 - t4352 - 0.11696447245269292414e1_f64 * t3617 + t2633 - 0.36622894612013090108e-3_f64 * t3640 - t2642 + t2651 + 0.19751673498613801407e-1_f64 * t4475 * t493 - t2678 - 8.0_f64 * t3643 - 8.0_f64 * t3645 + 2.0_f64 * t3650;
    t4481
}
