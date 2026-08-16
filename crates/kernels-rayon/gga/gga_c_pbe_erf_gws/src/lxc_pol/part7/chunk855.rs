//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 855/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk855(t5309: f64, t5312: f64, t1820: f64, t1885: f64, t5333: f64, t597: f64, t610: f64, t5019: f64, t5515: f64, t586: f64, t593: f64, t1656: f64, t5304: f64) -> (f64, f64, f64, f64, f64) {
    let t16515 = 16.0_f64 / 5.0_f64 * t5312 * t5309;
    let t16520 = 16.0_f64 / 15.0_f64 * t1820 * t1885 * t597 * t5333 * t610;
    let t16521 = t5312 * t5019;
    let t16522 = 64.0_f64 / 15.0_f64 * t16521;
    let t16523 = t5515 * t586;
    let t16525 = 32.0_f64 / 15.0_f64 * t16523 * t593;
    let t16527 = 16.0_f64 / 15.0_f64 * t5304 * t1656;
    (t16515, t16520, t16522, t16525, t16527)
}
