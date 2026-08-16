//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 493/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk493(t2156: f64, t2157: f64, t858: f64, t867: f64, t2155: f64, t837: f64, t863: f64, t864: f64) -> (f64, f64, f64, f64) {
    let t2158 = t2156 * t2157;
    let t2159 = t858 * t2158;
    let t2160 = t867 * t2159;
    let t2162 = t2155 * t2160 / 48.0_f64;
    let t2164 = t863 * t864 * t837;
    (t2158, t2160, t2162, t2164)
}
