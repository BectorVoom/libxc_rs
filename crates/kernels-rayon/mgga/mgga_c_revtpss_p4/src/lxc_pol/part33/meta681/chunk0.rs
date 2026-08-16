//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2221/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2221(t108879: f64, t2122: f64, t101237: f64, t101240: f64, t101243: f64, t104215: f64, t104226: f64, t108872: f64, t108876: f64, t108941: f64, t108945: f64, t1923: f64, t2123: f64, t26792: f64, t28154: f64, t29380: f64, t29532: f64, t30689: f64, t6954: f64, t7575: f64, t92568: f64, t96804: f64) -> f64 {
    let t111639 = t2122 * t108879;
    let t111652 = -t6954 * t30689 / 6.0_f64 - t1923 * t7575 * t29532 / 6.0_f64 - t1923 * t2122 * t108941 / 6.0_f64 + t108945 * t2123 / 3.0_f64 + 35.0_f64 * t96804 * t108872 - 10.0_f64 * t26792 * t108876 + 10.0_f64 * t92568 * t111639 - 10.0_f64 / 3.0_f64 * t101237 * t29380 - 10.0_f64 / 3.0_f64 * t101240 * t29380 - 10.0_f64 / 3.0_f64 * t101243 * t29380 - 10.0_f64 / 3.0_f64 * t28154 * t104215 - 10.0_f64 / 3.0_f64 * t28154 * t104226;
    t111652
}
