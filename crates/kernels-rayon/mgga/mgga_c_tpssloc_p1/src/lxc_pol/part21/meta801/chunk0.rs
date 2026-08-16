//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2788/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2788(t46376: f64, t16710: f64, t2663: f64, t41255: f64, t41259: f64, t46433: f64, t46435: f64, t46437: f64, t46439: f64, t16717: f64, t47176: f64, t157: f64, t46387: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t58983 = 0.11696447245269292414e1_f64 * t46376;
    let t58984 = t16710 * t2663;
    let t58985 = 0.24415263074675393405e-3_f64 * t58984;
    let t58986 = 0.5848223622634646207e0_f64 * t41255;
    let t58987 = 0.11696447245269292414e1_f64 * t41259;
    let t58988 = 0.11393789434848516923e-2_f64 * t46433;
    let t58989 = 0.97661052298701573622e-3_f64 * t46435;
    let t58990 = 4.0_f64 * t46437;
    let t58991 = 2.0_f64 * t46439;
    let t58993 = 48.0_f64 * t47176 * t16717;
    let t58994 = t46387 * t157;
    (t58983, t58985, t58986, t58987, t58988, t58989, t58990, t58991, t58993, t58994)
}
