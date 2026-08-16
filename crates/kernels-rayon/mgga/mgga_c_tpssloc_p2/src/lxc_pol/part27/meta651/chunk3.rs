//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2267/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2267(t1410: f64, t9239: f64, t2241: f64, t72: f64, t7431: f64, t12648: f64, t605: f64, t12652: f64, t12661: f64, t1865: f64, t26009: f64, t26070: f64, t26073: f64, t26076: f64, t6506: f64, t6510: f64, t83719: f64, t83827: f64, t83830: f64) -> f64 {
    let t90137 = t9239 * t1410;
    let t90141 = t72 * t7431 * t2241;
    let t90150 = t605 * t12648;
    let t90153 = t605 * t12652;
    let t90160 = t605 * t12661;
    let t90167 = 10.0_f64 * t90137 * t83719 + 35.0_f64 * t83830 * t90141 - 10.0_f64 * t83827 * t26009 + 2.0_f64 / 3.0_f64 * t26070 * t6506 + 2.0_f64 / 3.0_f64 * t26070 * t6510 + t90150 * t1865 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t90153 * t1865 + 2.0_f64 / 3.0_f64 * t26073 * t6506 + 2.0_f64 / 3.0_f64 * t26073 * t6510 + t90160 * t1865 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t26076 * t6506 + 2.0_f64 / 3.0_f64 * t26076 * t6510;
    t90167
}
