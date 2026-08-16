//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 791/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk791(t2153: f64, t837: f64, t863: f64, t2160: f64, t2289: f64, t2293: f64, t6247: f64, t904: f64, t916: f64, t2262: f64, t344: f64, t362: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6542 = t863 * t2153 * t837;
    let t6543 = t6542 * t2160;
    let t6544 = 7.0_f64 / 48.0_f64 * t6543;
    let t6545 = t2289 * t2293;
    let t6548 = t916 * t904 * t6247;
    let t6552 = 1.0_f64 / t2262 / t344;
    let t6553 = t6552 * t362;
    (t6542, t6544, t6545, t6548, t6552, t6553)
}
