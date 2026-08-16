//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 533/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk533(t2083: f64, t820: f64, t274: f64, t814: f64, t2255: f64, t2190: f64, t904: f64, t916: f64, t899: f64, t912: f64, t922: f64) -> (f64, f64, f64, f64, f64) {
    let t2278 = t820 * t2083;
    let t2279 = t274 * t814;
    let t2280 = t2278 * t2279;
    let t2281 = t2255 * t2280;
    let t2285 = t916 * t904 * t2190;
    let t2289 = t899 * t912 * t922;
    (t2278, t2279, t2281, t2285, t2289)
}
