//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 817/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk817(t6732: f64, t6733: f64, t6735: f64, t6736: f64, t339: f64, t338: f64, t376: f64, t2271: f64, t2365: f64, t822: f64, t833: f64, t2367: f64, t2397: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6738 = t6732 + t6733 + t6735 + t6736;
    let t6739 = t339 * t6738;
    let t6741 = t338 * t6739 * t376;
    let t6744 = t2271 * t2365;
    let t6745 = t822 * t6744;
    let t6746 = t6745 * t833;
    let t6748 = t2367 * t2397;
    (t6738, t6739, t6741, t6744, t6745, t6746, t6748)
}
