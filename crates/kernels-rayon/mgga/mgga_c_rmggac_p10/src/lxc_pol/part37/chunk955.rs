//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 955/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk955(t74953: f64, t74957: f64, t3351: f64, t498: f64, t515: f64, t7248: f64, t9523: f64, t9188: f64, t9527: f64, t71210: f64, t74961: f64, t74965: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t77287 = 0.2553875993597870364e-4_f64 * t74953;
    let t77288 = 0.7661627980793611092e-4_f64 * t74957;
    let t77292 = t3351 * t7248 * t515 * t9523 * t498;
    let t77293 = 0.12769379967989351819e-4_f64 * t77292;
    let t77296 = t3351 * t9188 * t515 * t9527;
    let t77297 = 0.25538759935978703638e-4_f64 * t77296;
    let t77299 = 0.36021158228745895953e-3_f64 * t71210;
    let t77300 = 0.20455996240684006298e-1_f64 * t74961;
    let t77301 = 0.2727466165424534173e-1_f64 * t74965;
    (t77287, t77288, t77293, t77297, t77299, t77300, t77301)
}
