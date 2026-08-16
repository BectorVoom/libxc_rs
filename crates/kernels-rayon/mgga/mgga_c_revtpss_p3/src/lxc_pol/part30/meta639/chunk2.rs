//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2218/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2218(t5: f64, t104194: f64, t104222: f64, t104249: f64, t104274: f64, t104303: f64, t104330: f64, t104359: f64, t104403: f64, t117: f64, t101504: f64, t101506: f64, t101508: f64, t101510: f64, t101512: f64, t101514: f64, t101517: f64, t101519: f64, t101521: f64, t101524: f64, t101526: f64, t101528: f64, t104163: f64) -> (f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t104407 = piecewise3(t8, 0.0_f64, t104194 + t104222 + t104249 + t104274 + t104303 + t104330 + t104359 + t104403);
    let t104408 = t104407 * t117;
    let t104409 = 2.0_f64 * t104163 + t104408 + t101504 + t101506 + t101508 + t101510 + t101512 + t101514 + t101517 + t101519 + t101521 + t101524 + t101526 + t101528;
    (t104408, t104409)
}
