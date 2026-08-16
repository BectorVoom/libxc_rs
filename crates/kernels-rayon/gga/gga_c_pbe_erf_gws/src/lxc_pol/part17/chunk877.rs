//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 877/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk877(t211: f64, t7570: f64, t2519: f64, t713: f64, t1888: f64, t7130: f64, t1652: f64, t2615: f64, t1009: f64, t4991: f64, t587: f64, t2815: f64, t586: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7572 = 8.0_f64 / 45.0_f64 * t211 * t7570;
    let t7573 = t2519 * t713;
    let t7576 = 8.0_f64 / 15.0_f64 * t7130 * t1888;
    let t7578 = 16.0_f64 / 135.0_f64 * t2615 * t1652;
    let t7579 = t4991 * t1009;
    let t7580 = t587 * t7579;
    let t7581 = 8.0_f64 / 405.0_f64 * t7580;
    let t7582 = t2815 * t586;
    (t7572, t7573, t7576, t7578, t7581, t7582)
}
