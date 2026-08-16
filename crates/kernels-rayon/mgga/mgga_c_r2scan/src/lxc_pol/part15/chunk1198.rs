//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1198/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1198(t3275: f64, t3352: f64, t40324: f64, t7088: f64, t797: f64, t3262: f64, t3263: f64, t114: f64, t1543: f64, t97: f64, t3575: f64, t481: f64, t7040: f64) -> (f64, f64, f64, f64) {
    let t40373 = t3275 * t40324 * t3352 / 2.0_f64;
    let t40374 = t797 * t7088;
    let t40377 = 3.0_f64 / 4.0_f64 * t3262 * t3263 * t40374;
    let t40379 = t97 * t1543 * t114;
    let t40381 = 3.0_f64 / 2.0_f64 * t40379 * t3575;
    let t40383 = t7040 * t481;
    (t40373, t40377, t40381, t40383)
}
