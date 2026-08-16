//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 780/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk780(t761: f64, t9494: f64, t116: f64, t229: f64, t597: f64, t60: f64, t59: f64, t212: f64, t2386: f64, t131: f64, t207: f64, t2559: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9496 = 0.10254018858216406658e4_f64 * t761 * t9494;
    let t9523 = t229 * t116;
    let t9533 = 1.0_f64 / t60 / t597;
    let t9534 = t59 * t9533;
    let t9537 = t2386 * t212;
    let t9538 = t116 * t131 * t9537;
    let t9540 = 0.13888888888888888889e-3_f64 * t9534 * t207 * t9538;
    let t9541 = t2559 * t786;
    (t9496, t9523, t9534, t9538, t9540, t9541)
}
