//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 274/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk274(t191: f64, t922: f64, t369: f64, t371: f64, t364: f64, t366: f64, t899: f64, t900: f64) -> (f64, f64, f64) {
    let t923 = t922 * t191;
    let t924 = t923 * t369;
    let t925 = t924 * t371;
    let t927 = 7.0_f64 / 4608.0_f64 * t364 * t925;
    let t929 = t899 * t900 * t366;
    (t923, t927, t929)
}
