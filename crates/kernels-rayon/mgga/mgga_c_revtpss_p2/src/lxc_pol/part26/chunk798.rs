//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 798/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk798(t114: f64, t10254: f64, t655: f64, t10201: f64, t10202: f64, t10204: f64, t10206: f64, t10210: f64, t10214: f64, t69: f64) -> f64 {
    let t115 = 1.0_f64 < t114;
    let t10255 = t655 * t10254;
    let t10259 = piecewise3(t115, 0.0_f64, -t10201 - 11.0_f64 / 3.0_f64 * t10202 - 2.0_f64 * t10204 + t10206 - 3.0_f64 / 4.0_f64 * t69 * t10210 + 3.0_f64 / 4.0_f64 * t69 * t10214 - t69 * t10255 / 8.0_f64);
    t10259
}
