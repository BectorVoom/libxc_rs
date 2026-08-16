//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1207/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1207(t94471: f64, t94473: f64, t94476: f64, t94483: f64, t94456: f64, t94460: f64, t94462: f64, t94464: f64, t94466: f64, t94468: f64, t94479: f64, t94481: f64, t94485: f64, t94487: f64) -> f64 {
    let t96321 = 455.0_f64 / 648.0_f64 * t94471;
    let t96322 = 0.51384669507166276316e-2_f64 * t94473;
    let t96323 = 0.3252886739816735289e-3_f64 * t94476;
    let t96326 = 0.18295201011342718161e-3_f64 * t94483;
    let t96329 = -0.24009450146119052704e-1_f64 * t94456 - 0.68026775414003982662e-1_f64 * t94460 - 0.85748036236139473944e-3_f64 * t94462 + 0.51448821741683684367e-1_f64 * t94464 - 0.85748036236139473944e-3_f64 * t94466 - 0.15246000842785598468e-3_f64 * t94468 - t96321 + t96322 - t96323 + 0.12196800674228478774e-3_f64 * t94479 + 3.0_f64 / 8.0_f64 * t94481 + t96326 + 7.0_f64 / 24.0_f64 * t94485 - t94487 / 24.0_f64;
    t96329
}
