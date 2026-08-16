//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 537/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk537(t1: f64, t2299: f64, t2182: f64, t904: f64, t2079: f64, t2081: f64, t2083: f64, t816: f64) -> (f64, f64, f64, f64) {
    let t2300 = t2299 * t1;
    let t2302 = t2300 * t904 * t2182;
    let t2305 = t2079 * t2081;
    let t2306 = t2083 * t816;
    (t2300, t2302, t2305, t2306)
}
