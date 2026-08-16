//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 880/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk880(t1047: f64, t7435: f64, t1031: f64, t1029: f64, t2727: f64, t441: f64, t2730: f64, t453: f64, t7307: f64, t450: f64, t2731: f64, t2723: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7436 = t7435 * t1047;
    let t7438 = 1.0_f64 * t1031 * t7436;
    let t7440 = 1.0_f64 / t2727 / t1029;
    let t7441 = t441 * t7440;
    let t7443 = 1.0_f64 / t2730 / t453;
    let t7444 = t7307 * t7443;
    let t7446 = 0.51726012919273400301e3_f64 * t7441 * t7444;
    let t7448 = 1.0_f64 / t2727 / t450;
    let t7449 = t441 * t7448;
    let t7450 = t7307 * t2731;
    let t7452 = 0.96491876992155210402e2_f64 * t7449 * t7450;
    let t7453 = t2723 * t2731;
    (t7436, t7438, t7440, t7441, t7443, t7444, t7446, t7448, t7449, t7450, t7452, t7453)
}
