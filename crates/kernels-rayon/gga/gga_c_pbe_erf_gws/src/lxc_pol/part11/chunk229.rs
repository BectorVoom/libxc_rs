//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 229/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk229(t24: f64, t713: f64, t712: f64, t395: f64, t401: f64) -> (f64, f64, f64) {
    let t714 = t24 * t713;
    let t716 = 0.60777777777777777777e-1_f64 * t712 * t714;
    let t719 = 0.32333333333333333333e-1_f64 * t395 - 0.11266666666666666667e0_f64 * t401;
    (t714, t716, t719)
}
