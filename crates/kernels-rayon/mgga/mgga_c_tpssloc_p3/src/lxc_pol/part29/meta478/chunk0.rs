//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1816/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1816(t491: f64, t7319: f64, t7287: f64, t3439: f64, t461: f64, t3243: f64, t7286: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24590 = t7319 * t491;
    let t24591 = t24590 * t7287;
    let t24594 = t3439 * t461;
    let t24595 = t24594 * t491;
    let t24596 = t7286 * t3243;
    let t24597 = t24595 * t24596;
    let t24600 = t461 * t491;
    let t24601 = t24600 * t225;
    (t24590, t24591, t24594, t24596, t24597, t24600, t24601)
}
