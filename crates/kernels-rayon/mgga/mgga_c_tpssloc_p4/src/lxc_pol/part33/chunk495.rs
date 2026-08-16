//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 495/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk495(t3034: f64, t335: f64, t368: f64, t1015: f64, t3033: f64, t1043: f64, t121: f64, t283: f64, t883: f64, t61: f64, t363: f64, t1017: f64, t67: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3036 = 1.0_f64 / t3034 / t335;
    let t3037 = t368 * t3036;
    let t3038 = t1015 * t3037;
    let t3039 = t3033 * t3038;
    let t3051 = t121 * t1043;
    let t3061 = 1.0_f64 / t283 / t883;
    let t3062 = t61 * t3061;
    let t3067 = t363 * t368;
    let t3068 = t1017 * t67;
    (t3036, t3037, t3038, t3039, t3051, t3061, t3062, t3067, t3068)
}
