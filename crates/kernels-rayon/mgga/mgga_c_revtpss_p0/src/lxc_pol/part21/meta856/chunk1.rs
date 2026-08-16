//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3248/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3248(t10321: f64, t13335: f64, t13340: f64, t13400: f64, t13405: f64, t1494: f64, t2251: f64, t2252: f64, t2291: f64, t2312: f64, t36: f64, t38: f64, t4181: f64, t4217: f64, t4218: f64, t4238: f64, t49889: f64, t60297: f64, t60330: f64, t627: f64, t641: f64, t70: f64, t85: f64) -> f64 {
    let t60360 = t38 * (t60297 + t60330) * t85 / 24.0_f64 + t13335 * t641 / 8.0_f64 - t2251 * t4217 * t85 / 4.0_f64 - t13340 * t641 / 4.0_f64 - t10321 * t1494 / 4.0_f64 - t2252 * t4238 / 4.0_f64 + t4218 * t2312 / 8.0_f64 - t4181 * t2291 * t85 / 4.0_f64 - t13400 * t641 / 2.0_f64 - t36 * t49889 * t70 * t85 / 12.0_f64 - t13405 * t627 * t85 / 4.0_f64;
    t60360
}
