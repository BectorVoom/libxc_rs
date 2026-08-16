//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 875/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk875(t59: f64, t9533: f64, t212: f64, t2386: f64, t116: f64, t131: f64, t207: f64, t2559: f64, t786: f64, t789: f64, t2563: f64, t2582: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9534 = t59 * t9533;
    let t9537 = t2386 * t212;
    let t9538 = t116 * t131 * t9537;
    let t9540 = 0.13888888888888888889e-3_f64 * t9534 * t207 * t9538;
    let t9541 = t2559 * t786;
    let t9542 = t9541 * t789;
    let t9544 = t2563 * t2582;
    (t9534, t9538, t9540, t9541, t9542, t9544)
}
