//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 623/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk623(t3088: f64, t3323: f64, t1167: f64, t2053: f64, t2455: f64, t950: f64) -> (f64, f64, f64, f64) {
    let t3324 = t3088 + t3323;
    let t3327 = t1167 * t2053;
    let t3341 = 0.82152657680133333336e0_f64 * t2455;
    let t3342 = t950 * t950;
    (t3324, t3327, t3341, t3342)
}
