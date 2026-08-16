//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2113/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2113(t1865: f64, t26045: f64, t26048: f64, t26070: f64, t26073: f64, t26076: f64, t27982: f64, t6492: f64, t6506: f64, t6510: f64, t7435: f64, t7442: f64, t96547: f64, t96551: f64, t96553: f64, t96556: f64, t96559: f64, t96562: f64) -> f64 {
    let t96579 = -5.0_f64 / 3.0_f64 * t96547 * t6492 + 2.0_f64 / 3.0_f64 * t96551 * t96553 + t96556 * t1865 / 3.0_f64 + t96559 * t1865 / 3.0_f64 + t96562 * t1865 / 3.0_f64 + t27982 * t6506 / 3.0_f64 + t27982 * t6510 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t26070 * t7442 + 2.0_f64 / 3.0_f64 * t26073 * t7442 + 2.0_f64 / 3.0_f64 * t26076 * t7442 + 2.0_f64 / 3.0_f64 * t7435 * t26045 + 2.0_f64 / 3.0_f64 * t7435 * t26048;
    t96579
}
