//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 762/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk762(t38: f64, t4217: f64, t1469: f64, t2299: f64, t4186: f64, t633: f64, t2306: f64, t637: f64, t606: f64, t77: f64, t1471: f64, t1487: f64, t1494: f64, t4182: f64, t4188: f64, t4191: f64, t4196: f64, t608: f64, t628: f64, t641: f64, t71: f64, t85: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4218 = t38 * t4217;
    let t4227 = t2299 * t1469;
    let t4230 = t633 * t4186;
    let t4232 = t2306 * t1469;
    let t4235 = t637 * t4186;
    let t4237 = 28.0_f64 / 9.0_f64 * t4227 * t606 - 4.0_f64 / 3.0_f64 * t4230 + 28.0_f64 / 9.0_f64 * t4232 * t606 + 4.0_f64 / 3.0_f64 * t4235;
    let t4238 = t77 * t4237;
    let t4241 = -t4182 * t85 / 12.0_f64 - t4188 * t85 / 12.0_f64 - t4191 * t85 / 12.0_f64 - t1471 * t641 / 12.0_f64 - t4196 * t85 / 12.0_f64 + t4218 * t85 / 24.0_f64 + t1487 * t641 / 24.0_f64 - t608 * t1494 / 12.0_f64 + t628 * t1494 / 24.0_f64 + t71 * t4238 / 24.0_f64;
    (t4218, t4227, t4232, t4237, t4238, t4241)
}
