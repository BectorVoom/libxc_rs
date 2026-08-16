//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 803/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk803(t1477: f64, t855: f64, t863: f64, t888: f64, t838: f64, t864: f64, t2264: f64, t899: f64, t922: f64, t2331: f64, t900: f64, t935: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6480 = t863 * t855 * t1477;
    let t6481 = t6480 * t888;
    let t6484 = t863 * t864 * t838;
    let t6501 = t899 * t2264 * t922;
    let t6505 = t899 * t900 * t2331;
    let t6506 = t6505 * t935;
    (t6480, t6481, t6484, t6501, t6505, t6506)
}
