//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1015/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1015(t11339: f64, t326: f64, t826: f64, t2365: f64, t3747: f64, t1114: f64, t833: f64, t1115: f64, t2397: f64, t2401: f64, t3207: f64, t335: f64, t3913: f64, t4487: f64, t844: f64, t8740: f64, t8745: f64, t8747: f64, t8751: f64, t9948: f64, t9953: f64, t9956: f64, t9958: f64, t9962: f64, t9965: f64, t9969: f64, t9973: f64, t9978: f64, param_a_c: f64) -> (f64, f64, f64, f64) {
    let t11340 = param_a_c * t11339;
    let t11341 = t326 * t11340;
    let t11342 = t11341 * t826;
    let t11347 = t3747 * t2365;
    let t11348 = t1114 * t11347;
    let t11349 = t11348 * t833;
    let t11351 = t3207 * t9948 / 8.0_f64 + t3913 * t2397 / 96.0_f64 - 7.0_f64 / 72.0_f64 * t9953 - t8740 - 7.0_f64 / 288.0_f64 * t9956 + t9958 * t833 / 96.0_f64 + 35.0_f64 / 432.0_f64 * t4487 - 7.0_f64 / 288.0_f64 * t9962 - t335 * t9965 / 96.0_f64 - t844 * t9969 / 48.0_f64 + t2401 * t9973 / 16.0_f64 - t8745 + 35.0_f64 / 216.0_f64 * t8747 - t335 * t9978 / 96.0_f64 + t11342 * t833 / 96.0_f64 + t1115 * t8751 / 48.0_f64 - 7.0_f64 / 288.0_f64 * t11349;
    (t11340, t11342, t11348, t11351)
}
