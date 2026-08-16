//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1135/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1135(t13396: f64, t70: f64, t4181: f64, t627: f64, t13312: f64, t36: f64, t4187: f64, t1470: f64, t2291: f64, t13389: f64, t13393: f64, t1494: f64, t2292: f64, t4182: f64, t4188: f64, t4191: f64, t4238: f64, t628: f64, t641: f64, t71: f64, t85: f64) -> f64 {
    let t13397 = t13396 * t70;
    let t13400 = t4181 * t627;
    let t13405 = t36 * t13312;
    let t13406 = t13405 * t70;
    let t13409 = t4187 * t627;
    let t13414 = t1470 * t2291;
    let t13419 = t2292 * t1494 / 24.0_f64 + t628 * t4238 / 12.0_f64 + t71 * t13389 / 24.0_f64 - t13393 * t85 / 12.0_f64 - t13397 * t85 / 6.0_f64 - t13400 * t85 / 6.0_f64 - t4182 * t641 / 6.0_f64 - t13406 * t85 / 12.0_f64 - t13409 * t85 / 6.0_f64 - t4188 * t641 / 6.0_f64 - t13414 * t85 / 12.0_f64 - t4191 * t641 / 6.0_f64;
    t13419
}
