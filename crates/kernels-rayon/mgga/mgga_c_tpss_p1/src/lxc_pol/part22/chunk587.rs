//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 587/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk587(t304: f64, t2551: f64, t2453: f64, t2455: f64, t2462: f64, t2467: f64, t2471: f64, t318: f64, t891: f64, t895: f64, t314: f64, t894: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2576 = t304 * t304;
    let t2577 = 1.0_f64 / t2576;
    let t2578 = t2551 * t2577;
    let t2581 = 0.12361111111111111111e-1_f64 * t2453;
    let t2586 = t2581 + 0.61805555555555555556e-2_f64 * t2455 - 0.61805555555555555555e-2_f64 * t2462 + 0.18541666666666666667e-1_f64 * t2467 - 0.92708333333333333333e-2_f64 * t2471;
    let t2587 = t2586 * t318;
    let t2589 = t891 * t895;
    let t2592 = t894 * t314;
    (t2576, t2577, t2578, t2581, t2586, t2587, t2589, t2592)
}
