//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1361/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1361(t5705: f64, t2815: f64, t41904: f64, t47787: f64, t59657: f64, t68442: f64, t76574: f64, t76578: f64, t76583: f64, t76587: f64, t76591: f64, t76595: f64, t76599: f64) -> (f64, f64, f64) {
    let t77041 = t5705 * t5705;
    let t77042 = t2815 * t77041;
    let t77058 = 112.0_f64 / 81.0_f64 * t47787 - 80.0_f64 / 81.0_f64 * t76574 - t76578 / 3.0_f64 - 16.0_f64 / 27.0_f64 * t59657 + 40.0_f64 / 9.0_f64 * t76583 - 20.0_f64 / 9.0_f64 * t76587 - 8.0_f64 * t76591 + 8.0_f64 * t76595 - 2.0_f64 / 3.0_f64 * t76599 + t41904 + 8.0_f64 / 3.0_f64 * t68442;
    (t77041, t77042, t77058)
}
