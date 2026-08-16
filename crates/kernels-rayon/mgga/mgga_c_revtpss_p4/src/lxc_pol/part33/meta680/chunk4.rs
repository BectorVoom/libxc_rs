//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2218/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2218(t60673: f64, t7565: f64, t13272: f64, t29411: f64, t104279: f64, t104282: f64, t108769: f64, t108792: f64, t108864: f64, t2123: f64, t26792: f64, t28133: f64, t29412: f64, t29562: f64, t30686: f64, t30689: f64, t6960: f64, t6963: f64, t7566: f64, t7706: f64, t96824: f64, t96827: f64) -> f64 {
    let t111532 = t60673 * t7565;
    let t111537 = t13272 * t29411;
    let t111548 = 5.0_f64 / 3.0_f64 * t29412 * t28133 + 2.0_f64 / 3.0_f64 * t6963 * t30686 + 5.0_f64 / 6.0_f64 * t7566 * t108792 + t6963 * t30689 / 3.0_f64 - 5.0_f64 * t96824 * t29562 + 5.0_f64 / 6.0_f64 * t111532 * t6960 + t108769 * t2123 / 3.0_f64 + 5.0_f64 / 3.0_f64 * t111537 * t6960 - 5.0_f64 * t96827 * t29562 - 5.0_f64 * t26792 * t108864 + 5.0_f64 / 3.0_f64 * t104279 * t7706 + 5.0_f64 / 3.0_f64 * t104282 * t7706;
    t111548
}
