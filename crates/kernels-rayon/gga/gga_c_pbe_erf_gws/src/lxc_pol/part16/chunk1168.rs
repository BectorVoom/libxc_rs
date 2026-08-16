//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1168/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1168(t3324: f64, t4120: f64, t360: f64, t898: f64, t2416: f64, t2100: f64, t376: f64, t2219: f64, t4383: f64, t4408: f64, t2387: f64, t6792: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15113 = t4120 * t3324;
    let t15636 = t898 * t360;
    let t15641 = t2416 * t360;
    let t19615 = t376 * t2100;
    let t19631 = t2219 * t898;
    let t19658 = t4408 * t4383;
    let t19704 = t2387 * t6792;
    (t15113, t15636, t15641, t19615, t19631, t19658, t19704)
}
