//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 679/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk679(t155: f64, t506: f64, t133: f64, t8199: f64, t1368: f64, t285: f64, t991: f64, t281: f64, t3013: f64, t545: f64, t39: f64, t159: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8236 = t155 * t506;
    let t8252 = t133 * t8199;
    let t8269 = t991 * t1368 * t285;
    let t8270 = t281 * t8269;
    let t8277 = t3013 * t545 * t285;
    let t8279 = t39 * t991;
    let t8281 = t8279 * t159 * t285;
    (t8236, t8252, t8269, t8270, t8277, t8279, t8281)
}
