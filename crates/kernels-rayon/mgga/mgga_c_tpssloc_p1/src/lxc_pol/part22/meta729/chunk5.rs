//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2395/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2395(t41959: f64, t59663: f64, t59665: f64, t59680: f64, t59688: f64, t59694: f64, t60204: f64, t68596: f64, t68599: f64, t68602: f64, t68605: f64, t68608: f64) -> f64 {
    let t68616 = 0.40256666666666666666e1_f64 * t68596 - 0.10064166666666666667e1_f64 * t68599 + 0.36231e1_f64 * t68602 - 0.10064166666666666667e1_f64 * t68605 - 0.543465e1_f64 * t68608 - 0.91983333333333333334e-1_f64 * t60204 - 0.60385000000000000002e0_f64 * t59663 + 0.20128333333333333334e0_f64 * t59665 + 0.30192500000000000001e0_f64 * t59680 + 0.80513333333333333334e0_f64 * t59688 - 0.40256666666666666668e0_f64 * t59694 + t41959;
    t68616
}
