//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 813/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk813(t761: f64, t9713: f64, t172: f64, t2448: f64, t763: f64, t177: f64, t2508: f64) -> (f64, f64, f64, f64) {
    let t9715 = 0.5848223622634646207e0_f64 * t761 * t9713;
    let t9716 = t2448 * t172;
    let t9717 = t9716 * t763;
    let t9718 = 0.17544670867903938621e1_f64 * t9717;
    let t9720 = 1.0_f64 / t2508 / t177;
    (t9715, t9716, t9718, t9720)
}
