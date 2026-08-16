//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1371/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1371(t41959: f64, t59688: f64, t59694: f64, t76574: f64, t76578: f64, t76583: f64, t76591: f64, t76599: f64, t76614: f64, t76622: f64, t76893: f64, t76896: f64, t76909: f64, t76915: f64) -> f64 {
    let t77204 = -0.99342e0_f64 * t76893 + 0.44152e0_f64 * t76896 + 0.198684e1_f64 * t76909 + 0.49671e0_f64 * t76915 - 0.89459259259259259259e0_f64 * t76574 - 0.301925e0_f64 * t76578 + 0.40256666666666666666e1_f64 * t76583 - 0.72462e1_f64 * t76591 - 0.60384999999999999999e0_f64 * t76599 + 0.72462e1_f64 * t76614 + 0.181155e1_f64 * t76622 + 0.16102666666666666667e1_f64 * t59688 - 0.80513333333333333336e0_f64 * t59694 + t41959;
    t77204
}
