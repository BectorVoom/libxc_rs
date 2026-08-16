//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 751/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk751(t1630: f64, t219: f64, t1811: f64, t1620: f64, t174: f64, t177: f64, t838: f64, t1243: f64, t574: f64, t1760: f64, t395: f64, t1766: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4934 = t1630 * t219;
    let t4935 = t4934 * t1811;
    let t4936 = t1620 * t4935;
    let t4939 = t174 * t838 * t177;
    let t4940 = 0.58774074074074074074e-2_f64 * t4939;
    let t4941 = t1243 * t574;
    let t4943 = t395 * t1760;
    let t4945 = t395 * t1766;
    (t4934, t4936, t4939, t4940, t4941, t4943, t4945)
}
