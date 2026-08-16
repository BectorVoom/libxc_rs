//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 798/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk798(t12339: f64, t4351: f64, t2477: f64, t3346: f64, t12345: f64, t476: f64, t12350: f64, t4366: f64, t2485: f64, t3354: f64, t12355: f64, t478: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12917 = t4351 * t12339;
    let t12919 = t2477 * t3346;
    let t12921 = t476 * t12345;
    let t12923 = t4366 * t12350;
    let t12925 = t2485 * t3354;
    let t12927 = t478 * t12355;
    (t12917, t12919, t12921, t12923, t12925, t12927)
}
