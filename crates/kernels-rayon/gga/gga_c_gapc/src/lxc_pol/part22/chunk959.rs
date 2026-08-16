//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 959/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk959(t11188: f64, t11189: f64, t3634: f64, t568: f64, t997: f64, t437: f64, t516: f64, t8356: f64, t125: f64, t515: f64, t619: f64, t2903: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11190 = t11188 * t11189;
    let t11192 = t3634 * t568;
    let t11193 = t997 * t11192;
    let t11195 = t516 * t437;
    let t11196 = t8356 * t11195;
    let t11198 = t515 * t125;
    let t11199 = t11198 * t619;
    let t11200 = t2903 * t11199;
    (t11190, t11192, t11193, t11195, t11196, t11198, t11199, t11200)
}
