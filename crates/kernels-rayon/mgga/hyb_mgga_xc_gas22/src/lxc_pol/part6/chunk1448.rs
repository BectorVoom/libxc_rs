//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1448/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1448(t1134: f64, t11461: f64, t11466: f64, t22746: f64, t22750: f64, t26416: f64, t26433: f64, t26437: f64, t26534: f64, t2889: f64, t2893: f64, t31480: f64, t31496: f64, t31501: f64, t31504: f64, t31539: f64, t31540: f64, t3788: f64, t4562: f64, t518: f64, t9632: f64, t9645: f64) -> f64 {
    let t31575 = 2800.0_f64 * t26416 * t31501 - 11200.0_f64 / 3.0_f64 * t26437 * t31504 - 256.0_f64 / 9.0_f64 * t9632 * t31480 + 400.0_f64 / 3.0_f64 * t26534 * t31501 - 800.0_f64 / 3.0_f64 * t26433 * t31504 - 400.0_f64 / 3.0_f64 * t26534 * t31504 + 400.0_f64 / 3.0_f64 * t26534 * t31496 + 320.0_f64 * t22746 * t31539 * t9645 - 448.0_f64 * t22750 * t31540 - 36.0_f64 * t1134 * t4562 * t2889 + 42.0_f64 * t518 * t11461 * t2893 - 8.0_f64 * t3788 * t11466;
    t31575
}
