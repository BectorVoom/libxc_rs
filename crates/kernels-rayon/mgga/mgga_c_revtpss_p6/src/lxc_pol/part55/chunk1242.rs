//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1242/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1242(t25082: f64, t33183: f64, t34301: f64, t22496: f64, t37318: f64, t128353: f64, t2056: f64, t128355: f64, t34258: f64, t7367: f64, t111176: f64, t28196: f64, t32577: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t128510 = 3.0_f64 * t25082 * t33183 * t34301;
    let t128513 = 3.0_f64 * t25082 * t37318 * t22496;
    let t128517 = 2.0_f64 * t128353 * t2056;
    let t128519 = 2.0_f64 * t128355 * t2056;
    let t128521 = 2.0_f64 * t34258 * t7367;
    let t128528 = 2.0_f64 * t28196 * t111176 * t32577;
    (t128510, t128513, t128517, t128519, t128521, t128528)
}
