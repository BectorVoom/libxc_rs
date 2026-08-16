//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 925/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk925(t114: f64, t22628: f64, t655: f64, t10201: f64, t13448: f64, t21818: f64, t21827: f64, t22590: f64, t22593: f64, t69: f64) -> f64 {
    let t115 = 1.0_f64 < t114;
    let t22629 = t655 * t22628;
    let t22633 = piecewise3(t115, 0.0_f64, -t10201 - 11.0_f64 / 3.0_f64 * t13448 - 2.0_f64 * t21818 + t21827 - 3.0_f64 / 4.0_f64 * t69 * t22590 + 3.0_f64 / 4.0_f64 * t69 * t22593 - t69 * t22629 / 8.0_f64);
    t22633
}
