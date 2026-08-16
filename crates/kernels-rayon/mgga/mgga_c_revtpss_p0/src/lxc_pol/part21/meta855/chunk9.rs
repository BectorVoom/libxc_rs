//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3244/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3244(t10416: f64, t118: f64, t13207: f64, t13521: f64, t13532: f64, t13540: f64, t1502: f64, t1519: f64, t18153: f64, t18163: f64, t2322: f64, t3813: f64, t4246: f64, t4254: f64, t4257: f64, t4292: f64, t46126: f64, t49851: f64, t49856: f64, t56137: f64, t60177: f64, t651: f64, t670: f64) -> f64 {
    let t60183 = -6.0_f64 * t2322 * t13521 - 6.0_f64 * t651 * t18153 * t670 - 12.0_f64 * t2322 * t13532 - 12.0_f64 * t4254 * t13532 - 6.0_f64 * t651 * t3813 * t4292 - 12.0_f64 * t2322 * t13540 - 2.0_f64 * t46126 * t1519 - 6.0_f64 * t49851 * t1519 - 6.0_f64 * t10416 * t4257 - 2.0_f64 * t49856 * t1519 - 6.0_f64 * t18163 * t4257 - t118 * (t56137 + t60177) - 3.0_f64 * t4246 * t3813 - t1502 * t13207;
    t60183
}
