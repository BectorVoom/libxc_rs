//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 732/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk732(t5990: f64, t5993: f64, t5994: f64, t5996: f64, t5999: f64, t6003: f64, t6005: f64, t6008: f64, t6012: f64, t6015: f64, t6018: f64, t6021: f64) -> f64 {
    let t6023 = -0.18903244333884670701e0_f64 * t5990 - t5993 + 0.94516221669423353502e-1_f64 * t5994 + 0.18903244333884670701e0_f64 * t5996 + t5999 + t6003 - t6005 + 0.19753890328909480882e-1_f64 * t6008 + t6012 + t6015 - 0.59261670986728442646e-2_f64 * t6018 - 0.11852334197345688529e-1_f64 * t6021;
    t6023
}
