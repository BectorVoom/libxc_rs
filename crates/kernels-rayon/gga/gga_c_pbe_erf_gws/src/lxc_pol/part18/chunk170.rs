//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 170/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk170(t156: f64, t386: f64, t4: f64, t407: f64, t435: f64, t442: f64, t450: f64, t457: f64, t71: f64, t84: f64) -> f64 {
    let t460 = 0.53236443333333333332e-3_f64 * t4 * t156 * t71 + 1.0_f64 * t435 * t442 - t386 - t407 + 0.18311555036753159941e-3_f64 * t4 * t156 * t84 + 0.58482233974552040708e0_f64 * t450 * t457;
    t460
}
