//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1228/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1228(t2427: f64, t9909: f64, t39568: f64, t761: f64, t2535: f64, t9716: f64, t39382: f64, t2531: f64, t9713: f64, t39302: f64, t39563: f64, t39585: f64, t39590: f64, t39593: f64, t40818: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t41251 = t2427 * t9909;
    let t41252 = 48.0_f64 * t41251;
    let t41254 = 0.14035736694323150897e2_f64 * t761 * t39568;
    let t41255 = t9716 * t2535;
    let t41256 = 0.35089341735807877242e1_f64 * t41255;
    let t41258 = 0.91082604192152556044e5_f64 * t761 * t39382;
    let t41259 = t2531 * t9713;
    let t41260 = 0.23392894490538584828e1_f64 * t41259;
    let t41262 = 0.5848223622634646207e0_f64 * t761 * t39302;
    let t41263 = t39563 - t40818 - t39585 + t39590 + t41252 - t39593 + t41254 - t41256 - t41258 - t41260 - t41262;
    (t41252, t41254, t41256, t41258, t41260, t41262, t41263)
}
