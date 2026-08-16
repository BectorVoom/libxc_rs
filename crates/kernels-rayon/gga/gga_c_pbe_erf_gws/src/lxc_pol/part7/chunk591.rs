//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 591/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk591(t2242: f64, t894: f64, t2367: f64, t2379: f64, t2352: f64, t810: f64, t2376: f64, t2409: f64, t2233: f64, t2246: f64, t1327: f64, t409: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4487 = t2242 * t894;
    let t4489 = t2367 * t2379;
    let t4491 = t2352 * t810;
    let t4493 = t2409 * t2376 * t4491;
    let t4496 = t2246 * t2233;
    let t4498 = t409 * t1327;
    (t4487, t4489, t4491, t4493, t4496, t4498)
}
