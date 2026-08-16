//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1119/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1119(t25220: f64, t25224: f64, t25225: f64, t25230: f64, t25232: f64, t25236: f64, t25238: f64, t25243: f64, t25246: f64, t25248: f64, t25285: f64) -> f64 {
    let t25286 = t25220 + t25224 - 0.17149607247227894789e-2_f64 * t25225 + t25230 - t25232 - t25236 + t25238 / 16.0_f64 + t25243 - 0.50820002809285328226e-4_f64 * t25246 + 0.85748036236139473945e-2_f64 * t25248 + t25285;
    t25286
}
