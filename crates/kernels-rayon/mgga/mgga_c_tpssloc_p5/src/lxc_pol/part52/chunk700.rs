//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 700/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk700(t605: f64, t608: f64, t38: f64, t43: f64, t625: f64, t44: f64, t607: f64, t614: f64) -> (f64, f64, f64, f64) {
    let t6495 = t605 * t608;
    let t6500 = t38 * t43;
    let t6503 = 8.0_f64 / 3.0_f64 * t625;
    let t6504 = -8.0_f64 / 3.0_f64 * t614 * t44 + 5.0_f64 / 6.0_f64 * t6500 * t607 + t6503;
    (t6495, t6500, t6503, t6504)
}
