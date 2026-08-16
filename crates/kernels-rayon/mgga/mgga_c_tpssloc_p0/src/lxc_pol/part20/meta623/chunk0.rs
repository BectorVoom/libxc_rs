//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2243/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2243(t1462: f64, t152: f64, t9288: f64, t4211: f64, t9874: f64, t13119: f64, t2663: f64, t2517: f64, t4098: f64, t1472: f64, t9862: f64, t41274: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46432 = 24.0_f64 * t9288 * t152 * t1462;
    let t46433 = t4211 * t9874;
    let t46434 = 0.56968947174242584612e-3_f64 * t46433;
    let t46435 = t13119 * t2663;
    let t46436 = 0.73245789224026180216e-3_f64 * t46435;
    let t46437 = t4098 * t2517;
    let t46438 = 3.0_f64 * t46437;
    let t46439 = t1472 * t9862;
    let t46444 = 0.35089341735807877242e1_f64 * t41274;
    (t46432, t46434, t46436, t46438, t46439, t46444)
}
