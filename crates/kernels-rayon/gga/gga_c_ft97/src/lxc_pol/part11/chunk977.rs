//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 977/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk977(t1546: f64, t89: f64, t9012: f64, t37401: f64, t9026: f64, t363: f64, t9348: f64, t2075: f64, t3139: f64, t583: f64, t143: f64, t37352: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40318 = t89 * t1546 * t9012;
    let t40321 = t89 * t37401 * t9026;
    let t40323 = t9348 * t363;
    let t40327 = t363 * t2075;
    let t40335 = t3139 * t583;
    let t40337 = t37352 * t143;
    (t40318, t40321, t40323, t40327, t40335, t40337)
}
