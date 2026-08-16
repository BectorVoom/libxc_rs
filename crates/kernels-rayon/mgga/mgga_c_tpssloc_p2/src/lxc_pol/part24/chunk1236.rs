//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1236/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1236(t2363: f64, t649: f64, t89: f64, t9416: f64, t88: f64, t2745: f64, t776: f64, t2553: f64, t868: f64, t2379: f64, t2749: f64, t2678: f64, t829: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t45637 = t649 * t2363;
    let t45640 = t89 * t9416;
    let t45814 = t88 * t9416;
    let t46240 = t2745 * t776;
    let t46252 = t2553 * t868;
    let t46298 = t2379 * t868;
    let t46320 = t776 * t2749;
    let t46362 = t2745 * t868;
    let t46511 = t829 * t2678;
    (t45637, t45640, t45814, t46240, t46252, t46298, t46320, t46362, t46511)
}
