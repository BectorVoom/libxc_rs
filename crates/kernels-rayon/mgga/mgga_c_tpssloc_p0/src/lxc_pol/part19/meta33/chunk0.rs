//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 239/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk239(t111: f64, t89: f64, t107: f64, t626: f64, t106: f64, t38: f64, t606: f64, tau0: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t652 = t89 * t111;
    let t654 = t626 * t107 / 3.0_f64;
    let t655 = t106 * t106;
    let t656 = 1.0_f64 / t655;
    let t657 = tau0 * t38;
    let t659 = t606 / 2.0_f64;
    (t652, t654, t655, t656, t657, t659)
}
