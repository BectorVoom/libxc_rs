//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2662/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2662(t2239: f64, t5385: f64, t12568: f64, t12582: f64, t12719: f64, t1437: f64, t16: f64, t19313: f64, t19445: f64, t2240: f64, t2241: f64, t2307: f64, t39033: f64, t39037: f64, t39043: f64, t39049: f64, t3953: f64, t3958: f64, t4021: f64, t45844: f64, t46099: f64, t5389: f64, t5445: f64, t645: f64, t86: f64, t9231: f64, t9239: f64) -> f64 {
    let t55921 = t5385 * t2239;
    let t55924 = -8.0_f64 * t46099 * t1437 - 16.0_f64 * t12568 * t4021 + 40.0_f64 * t2240 * t1437 * t12719 - 240.0_f64 * t45844 * t12582 - 120.0_f64 * t9239 * t5445 * t2241 - 8.0_f64 * t3953 * t12719 + 80.0_f64 * t9231 * t19313 + 40.0_f64 * t2240 * t19445 * t645 + 20.0_f64 * t2240 * t5445 * t2307 + 20.0_f64 * t39049 * t5389 - 480.0_f64 * t9239 * t3958 * t4021 + (-0.888e1_f64 * t16 + 678.0_f64 * t39033 - 0.52752e4_f64 * t39037 + t39043) * t86 + 20.0_f64 * t55921 * t2241;
    t55924
}
