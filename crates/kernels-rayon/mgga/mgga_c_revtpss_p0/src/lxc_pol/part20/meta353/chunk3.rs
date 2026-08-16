//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1287/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1287(t45: f64, t57: f64, t10326: f64, t10472: f64, t2251: f64, t2258: f64, t2299: f64, t39443: f64, t39449: f64, t39457: f64, t633: f64, t766: f64, t80: f64, t10481: f64, t2306: f64, t637: f64, t770: f64, t83: f64, zeta_threshold: f64) -> (f64, f64) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t39461 = piecewise3(t151, 0.0_f64, -56.0_f64 / 81.0_f64 * t2299 * t39443 + 16.0_f64 / 9.0_f64 * t633 * t2251 * t2258 - 2.0_f64 / 3.0_f64 * t80 * t39449 - 8.0_f64 / 9.0_f64 * t10472 * t10326 + 2.0_f64 / 3.0_f64 * t766 * t39457);
    let t39474 = piecewise3(t155, 0.0_f64, -56.0_f64 / 81.0_f64 * t2306 * t39443 - 16.0_f64 / 9.0_f64 * t637 * t2251 * t2258 - 2.0_f64 / 3.0_f64 * t83 * t39449 - 8.0_f64 / 9.0_f64 * t10481 * t10326 - 2.0_f64 / 3.0_f64 * t770 * t39457);
    (t39461, t39474)
}
