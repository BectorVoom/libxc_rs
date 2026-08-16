//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 767/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk767(t191: f64, t5060: f64, t1641: f64, t261: f64, t174: f64, t205: f64, t838: f64, t1243: f64, t628: f64, t1639: f64, t56: f64, t1672: f64, t662: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5061 = t191 * t5060;
    let t5063 = 1.0_f64 / t1641 / t261;
    let t5081 = t174 * t838 * t205;
    let t5082 = 0.11197407407407407407e0_f64 * t5081;
    let t5083 = t1243 * t628;
    let t5089 = t56 * t1639;
    let t5102 = t1672 * t662;
    (t5061, t5063, t5081, t5082, t5083, t5089, t5102)
}
