//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 854/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk854(t11007: f64, t383: f64, t1014: f64, t10471: f64, t10470: f64, t10481: f64, t381: f64, t360: f64, t6739: f64, t1057: f64, t10960: f64, t3120: f64, t3188: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11043 = t383 * t11007;
    let t11045 = t10471 * t1014;
    let t11046 = t10470 * t11045;
    let t11047 = t381 * t10481;
    let t11048 = t6739 * t360;
    let t11049 = t11047 * t11048;
    let t11051 = t10960 * t1057;
    let t11054 = t3188 * t3120;
    (t11043, t11046, t11047, t11049, t11051, t11054)
}
