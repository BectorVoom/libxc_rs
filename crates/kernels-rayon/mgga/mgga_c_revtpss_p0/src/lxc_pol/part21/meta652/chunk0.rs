//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2439/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2439(t225: f64, t42277: f64, t366: f64, t11792: f64, t3215: f64, t11951: f64, t3224: f64, t1025: f64, t11809: f64, t127: f64, t371: f64, t1053: f64, t11782: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42278 = t42277 * t225;
    let t42279 = t42278 * t366;
    let t42282 = t11792 * t3215;
    let t42284 = t3224 * t11951;
    let t42288 = t1025 * t371 * t127 * t11809;
    let t42290 = t11782 * t1053;
    (t42278, t42279, t42282, t42284, t42288, t42290)
}
