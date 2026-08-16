//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1092/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1092(t1025: f64, t11954: f64, t11942: f64, t1032: f64, t11878: f64, t9185: f64, t141: f64, t11932: f64, t11938: f64, t11952: f64, t9221: f64, t9223: f64, t9226: f64, t9228: f64) -> (f64, f64, f64, f64) {
    let t11955 = t1025 * t11954;
    let t11958 = 0.19931111111111111111e0_f64 * t11942;
    let t11960 = t1032 * t11954;
    let t11962 = t9185 * t11878;
    let t11963 = t141 * t11962;
    let t11965 = 0.26574814814814814816e0_f64 * t9221 + 0.66437037037037037038e-1_f64 * t9223 - 0.19931111111111111111e0_f64 * t9226 - 0.99655555555555555557e-1_f64 * t9228 + 0.36514074074074074074e-1_f64 * t11932 + 0.1898925e1_f64 * t11955 + 0.13287407407407407408e0_f64 * t11938 - t11958 + 0.29896666666666666667e0_f64 * t11952 + 0.3071625e0_f64 * t11960 + 0.36514074074074074075e-1_f64 * t11963;
    (t11955, t11960, t11963, t11965)
}
