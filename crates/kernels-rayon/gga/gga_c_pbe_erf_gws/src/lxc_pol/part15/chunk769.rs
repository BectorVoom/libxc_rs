//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 769/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk769(t1749: f64, t586: f64, t1630: f64, t1634: f64, t639: f64, t1639: f64, t9: f64, t1644: f64, t213: f64, t1623: f64, t1620: f64, t1673: f64, t579: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5470 = t1749 * t586;
    let t5477 = t1630 * t1634;
    let t5478 = t639 * t5477;
    let t5480 = t9 * t1639;
    let t5481 = t5480 * t1644;
    let t5482 = t639 * t5481;
    let t5493 = t9 * t213;
    let t5494 = t5493 * t1623;
    let t5495 = t1620 * t5494;
    let t5513 = t579 * t1673;
    (t5470, t5478, t5480, t5482, t5493, t5495, t5513)
}
