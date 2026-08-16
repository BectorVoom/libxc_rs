//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1232/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1232(t32636: f64, t7898: f64, t34266: f64, t7235: f64, t2033: f64, t28196: f64, t28286: f64, t5778: f64, t28177: f64, t8698: f64, t2014: f64, t33651: f64, t7536: f64) -> (f64, f64, f64, f64, f64) {
    let t128235 = t7898 * t32636;
    let t128236 = t7235 * t34266;
    let t128240 = 2.0_f64 * t28196 * t28286 * t2033 * t5778;
    let t128242 = 3.0_f64 * t8698 * t28177;
    let t128244 = t2014 * t7536 * t33651;
    (t128235, t128236, t128240, t128242, t128244)
}
