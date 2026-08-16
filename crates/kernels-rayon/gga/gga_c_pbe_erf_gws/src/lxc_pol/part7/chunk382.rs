//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 382/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk382(t510: f64, t513: f64, t137: f64, t512: f64, t131: f64, t520: f64) -> (f64, f64, f64, f64) {
    let t1572 = t510 * t513;
    let t1576 = 1.0_f64 / t512 / t137;
    let t1577 = t131 * t1576;
    let t1578 = t520 * t520;
    (t1572, t1576, t1577, t1578)
}
