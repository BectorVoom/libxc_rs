//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 837/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk837(t2627: f64, t5312: f64, t1022: f64, t1791: f64, t1793: f64, t1621: f64, t1620: f64, t1893: f64, t2612: f64, t1044: f64, t5109: f64, t639: f64) -> (f64, f64, f64, f64) {
    let t7026 = 8.0_f64 / 15.0_f64 * t5312 * t2627;
    let t7027 = t1791 * t1022;
    let t7028 = t7027 * t1793;
    let t7029 = t1621 * t7028;
    let t7031 = 8.0_f64 / 15.0_f64 * t1620 * t7029;
    let t7033 = 8.0_f64 / 45.0_f64 * t2612 * t1893;
    let t7035 = t5109 * t1044 * t1793;
    let t7036 = t1621 * t7035;
    let t7038 = 4.0_f64 / 5.0_f64 * t639 * t7036;
    (t7026, t7031, t7033, t7038)
}
