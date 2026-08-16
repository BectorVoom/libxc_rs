//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 804/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk804(t10288: f64, t2237: f64, t592: f64, t2236: f64, t3: f64, t25: f64, t88: f64, t89: f64, t90: f64, t29: f64, t46: f64, t47: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10289 = 540.0_f64 * t10288;
    let t10290 = t592 * t2237;
    let t10291 = 756.0_f64 * t10290;
    let t10292 = t2236 * t3;
    let t10293 = 1.0_f64 / t10292;
    let t10295 = 336.0_f64 * t25 * t10293;
    let t10308 = 1.0_f64 / t90 / t89 / t88;
    let t10309 = t29 * t10308;
    let t10355 = 1.0_f64 / t47 / t46;
    (t10289, t10291, t10295, t10308, t10309, t10355)
}
