//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 840/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk840(t4996: f64, t4987: f64, t7026: f64, t7031: f64, t7033: f64, t7038: f64, t7042: f64, t7045: f64, t7047: f64, t7054: f64, t7060: f64, t7067: f64, t7072: f64, t7074: f64, t7075: f64, t7077: f64, t7079: f64) -> (f64, f64) {
    let t7080 = 16.0_f64 / 135.0_f64 * t4996;
    let t7081 = -t7026 + t7031 - t7033 - t7038 + t7042 - t7045 + t7047 + t7054 - t7060 + t7067 - t7072 + t7074 + 4.0_f64 / 9.0_f64 * t7075 + t7077 - 2.0_f64 / 45.0_f64 * t4987 - t7079 - t7080;
    (t7080, t7081)
}
