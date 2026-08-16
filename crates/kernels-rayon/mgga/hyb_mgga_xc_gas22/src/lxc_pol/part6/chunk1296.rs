//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1296/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1296(t10437: f64, t136: f64, t550: f64, t24001: f64, t24003: f64, t24006: f64, t24011: f64, t24013: f64, t28086: f64, t28089: f64, t28092: f64, t28095: f64, t28097: f64, t28102: f64, t28104: f64, t28106: f64, t28108: f64, t28111: f64, t28115: f64, t28119: f64, t28121: f64, t675: f64, t684: f64, t687: f64) -> f64 {
    let t28125 = t136 * t550 * t10437;
    let t28128 = -t28086 / 48.0_f64 - 7.0_f64 / 48.0_f64 * t28089 - t28092 / 48.0_f64 - 7.0_f64 / 48.0_f64 * t28095 - t684 * t687 * t28097 * t675 / 32.0_f64 - 7.0_f64 / 16.0_f64 * t28102 - t28104 / 16.0_f64 - 7.0_f64 / 16.0_f64 * t28106 - t28108 / 32.0_f64 - t28111 / 32.0_f64 - 5.0_f64 / 144.0_f64 * t24001 + t28115 / 96.0_f64 - t24003 / 16.0_f64 + t24006 / 24.0_f64 + t28119 / 48.0_f64 + t28121 / 48.0_f64 + 41.0_f64 / 48.0_f64 * t24011 - t28125 / 16.0_f64 + t24013 / 8.0_f64;
    t28128
}
