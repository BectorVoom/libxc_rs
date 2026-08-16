//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1126/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1126(t39273: f64, t39275: f64, t39278: f64, t39281: f64, t39284: f64, t39289: f64, t39291: f64, t39293: f64, t39295: f64, t39298: f64, t683: f64, t702: f64) -> f64 {
    let t39563 = 1.0_f64 * t683 * (-0.21099166666666666667e1_f64 * t39273 + 0.202552e2_f64 * t39275 - 0.75019259259259259258e1_f64 * t39278 + 0.6564185185185185185e1_f64 * t39281 + 0.31003950617283950618e1_f64 * t39284 + 0.68258333333333333335e-1_f64 * t39289 - 0.10921333333333333333e1_f64 * t39291 + 0.12134814814814814815e1_f64 * t39293 + 0.10617962962962962963e1_f64 * t39295 + 0.13388493827160493828e1_f64 * t39298) * t702;
    t39563
}
