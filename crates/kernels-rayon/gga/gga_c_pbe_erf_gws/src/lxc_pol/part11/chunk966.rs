//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 966/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk966(t1114: f64, t20877: f64, t1109: f64, t2298: f64, t21497: f64, t1140: f64, t21511: f64, t1136: f64, t21253: f64, t21491: f64, t3179: f64, t21529: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27805 = t1114 * t20877;
    let t27917 = t1109 * t2298;
    let t28043 = t1114 * t21497;
    let t28074 = t21511 * t1140;
    let t28173 = t21253 * t1136;
    let t28195 = t21491 * t3179;
    let t28269 = t1114 * t21529;
    (t27805, t27917, t28043, t28074, t28173, t28195, t28269)
}
