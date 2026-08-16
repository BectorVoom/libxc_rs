//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1379/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1379(t119877: f64, t120002: f64, t120003: f64, t123119: f64, t123120: f64, t123122: f64, t123124: f64, t123126: f64, t123129: f64, t24999: f64, t26103: f64, t27879: f64, t6517: f64, t7271: f64, t7989: f64, t8329: f64) -> f64 {
    let t123137 = -2.0_f64 * t24999 * t7271 - 2.0_f64 * t26103 * t7989 - 2.0_f64 * t27879 * t6517 + t119877 + t120002 - t120003 - t123119 - 2.0_f64 * t123120 - 2.0_f64 * t123122 - 2.0_f64 * t123124 - 2.0_f64 * t123126 - 2.0_f64 * t123129 - t8329;
    t123137
}
