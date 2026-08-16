//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2339/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2339(t2110: f64, t24505: f64, t24508: f64, t26070: f64, t26073: f64, t26076: f64, t7256: f64, t7259: f64, t7435: f64, t90150: f64, t90153: f64, t90160: f64, t90343: f64) -> f64 {
    let t96021 = 2.0_f64 / 3.0_f64 * t90343 * t2110 + 2.0_f64 / 3.0_f64 * t26070 * t7256 + 2.0_f64 / 3.0_f64 * t26070 * t7259 + t90150 * t2110 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t90153 * t2110 + 2.0_f64 / 3.0_f64 * t26073 * t7256 + 2.0_f64 / 3.0_f64 * t26073 * t7259 + t90160 * t2110 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t26076 * t7256 + 2.0_f64 / 3.0_f64 * t26076 * t7259 + t7435 * t24505 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t7435 * t24508;
    t96021
}
