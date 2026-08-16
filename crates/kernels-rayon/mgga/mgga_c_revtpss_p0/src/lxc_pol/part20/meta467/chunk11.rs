//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1797/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1797(t13226: f64, t13250: f64, t1456: f64, t1458: f64, t1464: f64, t3: f64, t39397: f64, t39399: f64, t39401: f64, t39403: f64, t4154: f64, t4168: f64, t47693: f64, t47728: f64, t575: f64) -> f64 {
    let tv4rho40 = t3 * t47693 * t575 + 4.0_f64 * t13226 * t1464 + 4.0_f64 * t13250 * t1456 + t1458 * t47728 + 6.0_f64 * t4154 * t4168 + 4.0_f64 * t39397 + 12.0_f64 * t39399 + 12.0_f64 * t39401 + 4.0_f64 * t39403;
    tv4rho40
}
