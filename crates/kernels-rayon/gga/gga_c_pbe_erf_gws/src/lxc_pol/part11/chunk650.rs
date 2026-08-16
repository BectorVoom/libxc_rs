//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 650/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk650(t2365: f64, t885: f64, t346: f64, t4395: f64, t1: f64, t2298: f64, t253: f64, t320: f64, t368: f64, t191: f64, t6201: f64, t915: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6331 = t2365 * t885;
    let t6335 = t4395 * t346;
    let t6365 = t2298 * t1;
    let t6366 = t6365 * t253;
    let t6382 = 1.0_f64 / t368 / t320;
    let t6383 = t191 * t6382;
    let t6384 = t6383 * t1;
    let t6401 = t6201 * t915;
    (t6331, t6335, t6365, t6366, t6382, t6383, t6384, t6401)
}
