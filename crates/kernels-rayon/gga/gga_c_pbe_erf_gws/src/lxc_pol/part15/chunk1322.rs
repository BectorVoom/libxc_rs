//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1322/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1322(t14101: f64, t8842: f64, t4028: f64, t8856: f64, t14046: f64, t3184: f64, t51408: f64, t3148: f64, t3123: f64, t51309: f64, t14015: f64, t9467: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t54315 = t14101 * t8842;
    let t54317 = t4028 * t8856;
    let t54319 = t14046 * t3184;
    let t54320 = 7.0_f64 / 72.0_f64 * t54319;
    let t54321 = 35.0_f64 / 216.0_f64 * t51408;
    let t54322 = t14046 * t3148;
    let t54323 = 7.0_f64 / 72.0_f64 * t54322;
    let t54324 = t3123 * t51309;
    let t54326 = t14015 * t9467;
    (t54315, t54317, t54320, t54321, t54323, t54324, t54326)
}
