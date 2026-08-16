//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 818/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk818(t2402: f64, t338: f64, t892: f64, t2373: f64, t2384: f64, t2401: f64, t2408: f64, t335: f64, t6130: f64, t6135: f64, t6140: f64, t6145: f64, t6151: f64, t6156: f64, t6160: f64, t6164: f64, t6170: f64, t6173: f64, t6175: f64, t6726: f64, t6731: f64, t6741: f64, t6746: f64, t6748: f64, t827: f64) -> (f64, f64) {
    let t6751 = t338 * t892 * t2402;
    let t6754 = -t335 * t6130 / 16.0_f64 - t827 * t6135 / 8.0_f64 - t2408 * t6140 / 8.0_f64 + t827 * t6145 / 16.0_f64 + 3.0_f64 / 16.0_f64 * t827 * t6151 + 7.0_f64 / 96.0_f64 * t6156 + t6160 * t6164 / 48.0_f64 - t2384 * t2373 / 16.0_f64 - t335 * t6170 / 32.0_f64 - 7.0_f64 / 48.0_f64 * t6173 + 7.0_f64 / 96.0_f64 * t6175 - t335 * t6726 / 96.0_f64 - t6731 + t335 * t6741 / 96.0_f64 - 7.0_f64 / 96.0_f64 * t6746 - 7.0_f64 / 48.0_f64 * t6748 + 3.0_f64 / 16.0_f64 * t2401 * t6751;
    (t6751, t6754)
}
