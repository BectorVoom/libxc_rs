//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1017/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1017(t1144: f64, t3307: f64, t338: f64, t328: f64, t3780: f64, t2306: f64, t3074: f64, t1162: f64, t3200: f64, t3717: f64, t938: f64, t2376: f64, t2409: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11384 = t338 * t1144 * t3307;
    let t11387 = t3780 * t328;
    let t11388 = t2306 * t11387;
    let t11389 = t3074 * t11388;
    let t11393 = t338 * t3200 * t1162;
    let t11396 = t3717 * t938;
    let t11398 = t2409 * t2376 * t11396;
    (t11384, t11387, t11389, t11393, t11396, t11398)
}
