//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 790/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk790(t19: f64, t9223: f64, t83: f64, t84: f64, t85: f64, t24: f64, t41: f64, t42: f64, t53: f64, t54: f64, t2585: f64, t2769: f64, t73: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9225 = 0.75936e3_f64 * t19 * t9223;
    let t9238 = 1.0_f64 / t85 / t84 / t83;
    let t9239 = t24 * t9238;
    let t9287 = 1.0_f64 / t42 / t41;
    let t9300 = 1.0_f64 / t54 / t53;
    let t9311 = 1232.0_f64 / 27.0_f64 * t2585;
    let t9321 = 1.0_f64 / t73 / t2769;
    (t9225, t9238, t9239, t9287, t9300, t9311, t9321)
}
