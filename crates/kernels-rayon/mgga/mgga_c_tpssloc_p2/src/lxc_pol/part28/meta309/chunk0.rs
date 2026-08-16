//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1229/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1229(t10947: f64, t3185: f64, t3199: f64, t1014: f64, t10471: f64, t10470: f64, t1057: f64, t10960: f64, t3120: f64, t3188: f64, t10474: f64, t10482: f64, t6739: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11034 = t10947 * t3185;
    let t11037 = t10947 * t3199;
    let t11045 = t10471 * t1014;
    let t11046 = t10470 * t11045;
    let t11051 = t10960 * t1057;
    let t11054 = t3188 * t3120;
    let t11058 = t10471 * t10474;
    let t11059 = t10470 * t11058;
    let t11060 = t6739 * t10482;
    (t11034, t11037, t11046, t11051, t11054, t11059, t11060)
}
