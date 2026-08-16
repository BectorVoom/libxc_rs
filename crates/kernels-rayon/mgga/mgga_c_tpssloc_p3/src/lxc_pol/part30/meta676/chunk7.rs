//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2114/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2114(t26021: f64, t26025: f64, t26028: f64, t26045: f64, t26051: f64, t26063: f64, t26070: f64, t26073: f64, t26076: f64, t27979: f64, t6506: f64, t6510: f64, t7428: f64, t7432: f64, t7435: f64, t7442: f64, t7446: f64, t90182: f64, t90185: f64) -> f64 {
    let t96605 = 2.0_f64 / 3.0_f64 * t26070 * t7446 + 2.0_f64 / 3.0_f64 * t26073 * t7446 + 2.0_f64 / 3.0_f64 * t26076 * t7446 + 2.0_f64 / 3.0_f64 * t7435 * t26021 + 2.0_f64 / 3.0_f64 * t7435 * t26025 + 5.0_f64 / 3.0_f64 * t90182 * t7432 + 5.0_f64 / 3.0_f64 * t90185 * t7432 + 5.0_f64 / 3.0_f64 * t26051 * t26063 + t27979 * t6506 / 3.0_f64 + t27979 * t6510 / 3.0_f64 - t26028 * t7442 / 3.0_f64 - t7428 * t26045 / 3.0_f64;
    t96605
}
