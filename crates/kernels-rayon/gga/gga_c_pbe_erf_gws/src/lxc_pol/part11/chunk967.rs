//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 967/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk967(t1114: f64, t21764: f64, t19810: f64, t1120: f64, t21681: f64, t1164: f64, t6729: f64, t1150: f64, t21117: f64, t1112: f64, t19561: f64, t1154: f64, t20646: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28394 = t1114 * t21764;
    let t28397 = t1114 * t19810;
    let t28413 = t21681 * t1120;
    let t28487 = t6729 * t1164;
    let t28923 = t21117 * t1150;
    let t28975 = t1112 * t19561;
    let t29599 = t20646 * t1154;
    (t28394, t28397, t28413, t28487, t28923, t28975, t29599)
}
