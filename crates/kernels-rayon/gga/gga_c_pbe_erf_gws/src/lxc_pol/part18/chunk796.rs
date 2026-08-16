//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 796/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk796(t2106: f64, t814: f64, t816: f64, t322: f64, t897: f64) -> (f64, f64, f64, f64) {
    let t6089 = t814 * t2106;
    let t6094 = t816 * t816;
    let t6095 = 1.0_f64 / t6094;
    let t6096 = t322 * t6095;
    let t6125 = t897 * t897;
    let t6126 = 1.0_f64 / t6125;
    (t6089, t6096, t6125, t6126)
}
