//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 586/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk586(t1144: f64, t845: f64, t338: f64, t1118: f64, t892: f64, t2494: f64, t376: f64, t353: f64, t1162: f64, t1112: f64, t339: f64, t2306: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3089 = t1144 * t845;
    let t3090 = t338 * t3089;
    let t3093 = t892 * t1118;
    let t3094 = t338 * t3093;
    let t3097 = t376 * t2494;
    let t3098 = t353 * t3097;
    let t3099 = t338 * t3098;
    let t3102 = t892 * t1162;
    let t3103 = t338 * t3102;
    let t3106 = t1112 * t339;
    let t3107 = t2306 * t3106;
    (t3089, t3090, t3093, t3094, t3097, t3098, t3099, t3102, t3103, t3106, t3107)
}
