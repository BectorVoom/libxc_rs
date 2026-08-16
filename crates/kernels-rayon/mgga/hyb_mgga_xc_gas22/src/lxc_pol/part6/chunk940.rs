//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 940/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk940(t3138: f64, t3140: f64, t3142: f64, t3150: f64, t684: f64, t687: f64, t8476: f64, t8479: f64, t8481: f64, t8485: f64, t8491: f64, t8493: f64, t8501: f64, t8502: f64, t8506: f64, t8511: f64, t8513: f64, t8514: f64, t8519: f64, t8521: f64, t8526: f64) -> f64 {
    let t8530 = -t8476 - 7.0_f64 / 96.0_f64 * t8479 - t684 * t687 * t8481 / 64.0_f64 + t684 * t3150 * t8485 / 16.0_f64 - t8491 - t684 * t687 * t8493 / 32.0_f64 - t8501 - t3138 * t8502 * t3142 / 24.0_f64 - t3138 * t3140 * t8506 / 48.0_f64 - 7.0_f64 / 144.0_f64 * t8511 * t8513 * t8514 + t3138 * t8519 * t8521 / 12.0_f64 + t8526 * t3140 * t8514 / 16.0_f64;
    t8530
}
