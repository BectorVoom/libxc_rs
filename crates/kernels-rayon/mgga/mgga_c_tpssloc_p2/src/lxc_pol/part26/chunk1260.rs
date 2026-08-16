//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1260/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1260(t510: f64, t652: f64, t81455: f64, t1983: f64, t22584: f64, t22591: f64, t25014: f64, t9616: f64, t25373: f64, t46320: f64, t193: f64, t201: f64, t6665: f64) -> (f64, f64, f64, f64, f64) {
    let t81458 = 2.0_f64 * t652 * t510 * t81455;
    let t81469 = 9.0_f64 * t1983 * t22591 * t22584;
    let t81470 = t25014 * t9616;
    let t81476 = t25373 * t46320;
    let t81483 = t193 * t201 * t6665;
    (t81458, t81469, t81470, t81476, t81483)
}
