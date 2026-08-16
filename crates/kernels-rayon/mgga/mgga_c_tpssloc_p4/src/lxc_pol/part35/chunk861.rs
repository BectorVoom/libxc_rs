//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 861/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk861(t1014: f64, t10471: f64, t10470: f64, t360: f64, t6739: f64, t10474: f64, t10482: f64, t3127: f64, t3131: f64, t3215: f64, t390: f64, t268: f64, t405: f64, t6546: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11045 = t10471 * t1014;
    let t11046 = t10470 * t11045;
    let t11048 = t6739 * t360;
    let t11058 = t10471 * t10474;
    let t11059 = t10470 * t11058;
    let t11060 = t6739 * t10482;
    let t11064 = t10471 * t3127;
    let t11065 = t10470 * t11064;
    let t11066 = t6739 * t3131;
    let t11094 = 1.0_f64 / t3215 / t390;
    let t11135 = t268 * t6546 * t405;
    (t11046, t11048, t11059, t11060, t11065, t11066, t11094, t11135)
}
