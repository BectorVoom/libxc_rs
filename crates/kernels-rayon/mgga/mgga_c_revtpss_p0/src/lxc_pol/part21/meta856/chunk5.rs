//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3252/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3252(t10381: f64, t10407: f64, t13389: f64, t13392: f64, t13393: f64, t13396: f64, t13397: f64, t1487: f64, t1494: f64, t2292: f64, t4238: f64, t53459: f64, t53464: f64, t54450: f64, t60479: f64, t627: f64, t628: f64, t641: f64, t70: f64, t71: f64, t77: f64, t85: f64) -> f64 {
    let t60483 = -t54450 * t70 * t85 / 12.0_f64 - t53464 * t70 * t85 / 4.0_f64 - t13392 * t627 * t85 / 4.0_f64 - t13393 * t641 / 4.0_f64 - t53459 * t70 * t85 / 4.0_f64 - t13396 * t627 * t85 / 2.0_f64 - t13397 * t641 / 2.0_f64 + t1487 * t10407 / 24.0_f64 + t10381 * t1494 / 24.0_f64 + t2292 * t4238 / 8.0_f64 + t628 * t13389 / 8.0_f64 + t71 * t77 * t60479 / 24.0_f64;
    t60483
}
