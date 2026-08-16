//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 735/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk735(t154: f64, t9576: f64, t2588: f64, t21: f64, t59: f64, t207: f64, t795: f64, t4127: f64, t787: f64, t9526: f64, t9529: f64, t9540: f64, t9542: f64, t9544: f64, t9547: f64, t9552: f64, t9556: f64, t9559: f64, t9561: f64, t9566: f64, t9572: f64, t9574: f64) -> (f64, f64, f64) {
    let t9577 = t9576 * t154;
    let t9579 = 0.99999999999999999997e-2_f64 * t9577 * t2588;
    let t9580 = t59 * t21;
    let t9583 = 0.16435185185185185185e-1_f64 * t9580 * t207 * t795;
    let t9584 = 0.49999999999999999998e-2_f64 * t9526 - 0.16666666666666666666e-2_f64 * t787 * t9529 - t9540 - 0.38888888888888888888e-1_f64 * t9542 + 0.11666666666666666666e-1_f64 * t9544 - 0.15833333333333333333e-1_f64 * t9547 - 0.74999999999999999997e-2_f64 * t9552 + 0.24999999999999999999e-2_f64 * t9556 - 0.19999999999999999999e-1_f64 * t9559 * t9561 + 0.14999999999999999999e-1_f64 * t4127 * t9566 - t9572 - 0.34999999999999999998e-1_f64 * t9574 + t9579 - t9583;
    (t9577, t9580, t9584)
}
