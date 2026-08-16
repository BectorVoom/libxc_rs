//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 323/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk323(t898: f64, t938: f64, t353: f64, t338: f64, t335: f64, t827: f64, t833: f64, t842: f64, t844: f64, t847: f64, t894: f64) -> (f64, f64, f64, f64) {
    let t939 = t898 * t938;
    let t940 = t353 * t939;
    let t941 = t338 * t940;
    let t944 = t827 * t833 / 96.0_f64 - t842 - t844 * t847 / 48.0_f64 + t335 * t894 / 96.0_f64 - t335 * t941 / 96.0_f64;
    (t939, t940, t941, t944)
}
