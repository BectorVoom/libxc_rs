//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 590/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk590(t2382: f64, t4473: f64, t833: f64, t2222: f64, t840: f64, t814: f64, t898: f64, t938: f64, t353: f64, t859: f64) -> (f64, f64, f64, f64, f64) {
    let t4474 = t2382 * t4473;
    let t4475 = t4474 * t833;
    let t4477 = t840 * t2222;
    let t4482 = t898 * t814 * t938;
    let t4483 = t353 * t4482;
    let t4484 = t859 * t4483;
    (t4474, t4475, t4477, t4482, t4484)
}
