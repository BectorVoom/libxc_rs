//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1274/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1274(t1076: f64, t1123: f64, t833: f64, t837: f64, t850: f64, t14677: f64, t2503: f64, t11396: f64, t13888: f64, t2408: f64, t52931: f64, t53083: f64, t53779: f64, t56110: f64, t56113: f64, t56116: f64, t56119: f64, t56124: f64, t56126: f64, t56129: f64, t56133: f64, t56142: f64, t6793: f64, t8793: f64, t9283: f64) -> f64 {
    let t56147 = t850 * t1123 * t1076 * t837 * t833;
    let t56153 = t14677 * t2503;
    let t56155 = -t56110 / 48.0_f64 - t56113 / 48.0_f64 + t56116 / 48.0_f64 + t56119 / 16.0_f64 + t56124 / 96.0_f64 - 7.0_f64 / 1152.0_f64 * t56126 - t56129 / 768.0_f64 - t6793 * t56133 / 12.0_f64 - t8793 * t53083 / 12.0_f64 - t8793 * t53779 / 12.0_f64 - 7.0_f64 / 288.0_f64 * t56142 - 7.0_f64 / 288.0_f64 * t56147 - t52931 - t2408 * t9283 * t13888 * t11396 / 24.0_f64 + t56153 / 48.0_f64;
    t56155
}
