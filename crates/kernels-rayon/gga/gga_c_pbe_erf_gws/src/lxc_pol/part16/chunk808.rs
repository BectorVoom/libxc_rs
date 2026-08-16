//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 808/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk808(t2250: f64, t6274: f64, t810: f64, t875: f64, t824: f64, t745: f64, t874: f64, t343: f64, t2189: f64, t274: f64, t2145: f64, t2387: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6275 = t2250 * t6274;
    let t6277 = t875 * t810;
    let t6278 = t824 * t6277;
    let t6296 = t745 * t874;
    let t6297 = t6296 * t343;
    let t6303 = t274 * t2189 * t343;
    let t6322 = t2387 * t2145;
    (t6275, t6277, t6278, t6297, t6303, t6322)
}
