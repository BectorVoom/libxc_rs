//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1007/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1007(t493: f64, t7426: f64, t7438: f64, t7446: f64, t7452: f64, t7456: f64, t7459: f64, t7463: f64, t7466: f64, t7496: f64, t7498: f64, t7503: f64, t7506: f64, t7509: f64, t7512: f64, t7518: f64, t9369: f64) -> f64 {
    let t9390 = t7496 + 0.21687162600603479684e-1_f64 * t7498 + t7426 - t7503 + 40.0_f64 * t7506 + t7438 + t7446 - t7452 + 0.19751673498613801407e-1_f64 * t9369 * t493 + t7509 + t7456 - t7459 - t7463 - 0.18311447306006545054e-3_f64 * t7512 - t7466 - t7518;
    t9390
}
