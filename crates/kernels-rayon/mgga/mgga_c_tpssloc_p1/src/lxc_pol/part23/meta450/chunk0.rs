//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1297/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1297(t20825: f64, t46387: f64, t67099: f64, t46196: f64, t5660: f64, t193: f64, t202: f64, t2752: f64, t39316: f64, t39320: f64, t39373: f64, t39397: f64, t39400: f64, t39408: f64, t39411: f64, t40679: f64, t40685: f64, t40708: f64) -> (f64, f64, f64, f64) {
    let t75854 = 96.0_f64 * t46387 * t20825;
    let t75855 = 0.23392894490538584828e1_f64 * t67099;
    let t75856 = 0.14035736694323150897e2_f64 * t46196;
    let t75857 = t5660 * t5660;
    let t75862 = -3.0_f64 * t193 * t202 * t2752 * t75857 + t39316 + t39320 + t39373 - t39397 - t39400 + t39408 + t39411 - t40679 - t40685 + t40708 + t75854 - t75855 + t75856;
    (t75854, t75855, t75856, t75862)
}
