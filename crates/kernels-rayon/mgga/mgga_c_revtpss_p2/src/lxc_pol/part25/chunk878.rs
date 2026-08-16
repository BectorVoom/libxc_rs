//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 878/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk878(t10406: f64, t77: f64, t10317: f64, t10318: f64, t10321: f64, t10328: f64, t10331: f64, t10336: f64, t10381: f64, t2252: f64, t2260: f64, t2263: f64, t2292: f64, t2312: f64, t608: f64, t628: f64, t641: f64, t71: f64, t85: f64) -> f64 {
    let t10407 = t77 * t10406;
    let t10410 = -t10317 * t10318 / 4.0_f64 - t10321 * t85 / 4.0_f64 - t2252 * t641 / 4.0_f64 - t10328 * t85 / 12.0_f64 - t10331 * t85 / 4.0_f64 - t2260 * t641 / 4.0_f64 - t10336 * t85 / 4.0_f64 - t2263 * t641 / 2.0_f64 - t608 * t2312 / 4.0_f64 + t10381 * t85 / 24.0_f64 + t2292 * t641 / 8.0_f64 + t628 * t2312 / 8.0_f64 + t71 * t10407 / 24.0_f64;
    t10410
}
