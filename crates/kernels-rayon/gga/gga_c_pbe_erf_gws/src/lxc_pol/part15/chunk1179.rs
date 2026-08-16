//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1179/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1179(t326: f64, t825: f64, t6148: f64, t3067: f64, t830: f64, t9550: f64, t9607: f64, t2494: f64, t3222: f64, t28667: f64, t9370: f64, t9380: f64, param_a_c: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t36199 = t326 * t825;
    let t36200 = t36199 * t6148;
    let t36201 = t830 * t3067;
    let t36865 = t9607 * t9550;
    let t36888 = t2494 * param_a_c;
    let t36889 = t36888 * t3222;
    let t37214 = t28667 * t9370;
    let t38360 = t9607 * t9380;
    (t36199, t36200, t36201, t36865, t36889, t37214, t38360)
}
