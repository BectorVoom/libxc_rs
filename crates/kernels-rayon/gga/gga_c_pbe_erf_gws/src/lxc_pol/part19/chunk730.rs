//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 730/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk730(t2376: f64, t2409: f64, t4207: f64, t1144: f64, t1206: f64, t338: f64, t1161: f64, t1205: f64) -> (f64, f64, f64) {
    let t4209 = t2409 * t2376 * t4207;
    let t4212 = t1144 * t1206;
    let t4213 = t338 * t4212;
    let t4216 = t1205 * t1161;
    (t4209, t4213, t4216)
}
