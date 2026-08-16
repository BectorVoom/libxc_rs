//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1208/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1208(t1109: f64, t814: f64, t11731: f64, t5: f64, t343: f64, t2494: f64, t3178: f64, t3717: f64, t3802: f64, t6469: f64, t11806: f64, t810: f64, param_a_c: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t36897 = t1109 * t814;
    let t37440 = t5 * t11731;
    let t37441 = t37440 * t343;
    let t37454 = t3178 * t2494;
    let t37632 = t3717 * param_a_c;
    let t38036 = t6469 * t3802;
    let t38133 = t11806 * t810;
    (t36897, t37441, t37454, t37632, t38036, t38133)
}
