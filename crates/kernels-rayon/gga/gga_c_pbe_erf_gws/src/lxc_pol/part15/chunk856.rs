//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 856/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk856(t1413: f64, t7282: f64, t5089: f64, t11: f64, t2715: f64, t401: f64, t2712: f64, t1714: f64, t7097: f64, t5061: f64, t7212: f64, t657: f64, t7264: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7283 = t7282 * t1413;
    let t7284 = t5089 * t7283;
    let t7285 = t11 * t7284;
    let t7288 = 0.17777777777777777778e-1_f64 * t401 * t2715;
    let t7290 = 0.2962962962962962963e-2_f64 * t401 * t2712;
    let t7291 = t1714 * t7097;
    let t7294 = t5061 * t7283;
    let t7297 = t1714 * t7212;
    let t7300 = t657 * t7264;
    (t7283, t7285, t7288, t7290, t7291, t7294, t7297, t7300)
}
