//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 793/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk793(t346: f64, t6158: f64, t822: f64, t5: f64, t6161: f64, t337: f64, t2121: f64, t2100: f64, t274: f64, t2255: f64, t2278: f64, t2251: f64, t2299: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6566 = t6158 * t346;
    let t6567 = t822 * t6566;
    let t6568 = t5 * t6161;
    let t6569 = t337 * t6568;
    let t6570 = t2121 * t6569;
    let t6572 = t6567 * t6570 / 48.0_f64;
    let t6573 = t274 * t2100;
    let t6575 = t2255 * t2278 * t6573;
    let t6578 = t2251 * t2299;
    (t6566, t6567, t6568, t6569, t6570, t6572, t6573, t6575, t6578)
}
