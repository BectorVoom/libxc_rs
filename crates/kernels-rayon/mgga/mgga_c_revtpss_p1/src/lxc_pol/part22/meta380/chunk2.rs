//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1940/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1940(t13509: f64, t655: f64, t10201: f64, t10202: f64, t10204: f64, t10206: f64, t13448: f64, t13451: f64, t13453: f64, t13455: f64, t13459: f64, t13462: f64, t69: f64) -> (f64, f64) {
    let t13510 = t655 * t13509;
    let t13513 = -t10201 - 22.0_f64 / 9.0_f64 * t10202 - 2.0_f64 / 3.0_f64 * t10204 + t10206 / 3.0_f64 - 11.0_f64 / 9.0_f64 * t13448 - t13451 + t13453 - 3.0_f64 / 4.0_f64 * t69 * t13455 + t69 * t13459 / 2.0_f64 + t69 * t13462 / 4.0_f64 - t69 * t13510 / 8.0_f64;
    (t13510, t13513)
}
