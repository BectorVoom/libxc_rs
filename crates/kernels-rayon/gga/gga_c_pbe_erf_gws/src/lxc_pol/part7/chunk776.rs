//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 776/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk776(t6384: f64, t6385: f64, t904: f64, t2313: f64, t814: f64, t2278: f64, t2255: f64, t2156: f64, t274: f64, t343: f64, t851: f64, t6201: f64, t915: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6387 = t6384 * t904 * t6385;
    let t6390 = t2313 * t814;
    let t6391 = t2278 * t6390;
    let t6392 = t2255 * t6391;
    let t6395 = t274 * t2156;
    let t6396 = t6395 * t343;
    let t6398 = t2255 * t851 * t6396;
    let t6401 = t6201 * t915;
    (t6387, t6390, t6392, t6395, t6396, t6398, t6401)
}
