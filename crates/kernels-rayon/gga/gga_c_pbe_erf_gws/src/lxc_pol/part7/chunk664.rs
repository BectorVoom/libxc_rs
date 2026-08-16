//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 664/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk664(t5284: f64, t587: f64, t4360: f64, t591: f64, t590: f64, t1764: f64, t187: f64, t22: f64, t197: f64, t4951: f64, t4352: f64, t1802: f64, t1804: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5285 = t587 * t5284;
    let t5286 = 8.0_f64 / 27.0_f64 * t5285;
    let t5287 = t591 * t4360;
    let t5288 = t590 * t5287;
    let t5290 = 4.0_f64 / 45.0_f64 * t587 * t5288;
    let t5292 = 1.0_f64 / t187 / t1764;
    let t5293 = t22 * t5292;
    let t5294 = t197 * t4951;
    let t5295 = t5294 * t4352;
    let t5296 = t5293 * t5295;
    let t5298 = 32.0_f64 / 81.0_f64 * t587 * t5296;
    let t5299 = t1802 * t1804;
    (t5286, t5287, t5288, t5290, t5292, t5293, t5294, t5295, t5296, t5298, t5299)
}
