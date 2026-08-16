//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1280/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1280(t20022: f64, t20070: f64, t20073: f64, t23100: f64, t23102: f64, t23105: f64, t23108: f64, t23111: f64, t23113: f64, t23116: f64, t23118: f64, t23120: f64, t23124: f64, t23128: f64, t23139: f64, t23572: f64, t23575: f64, t7831: f64, t9858: f64) -> f64 {
    let t27706 = -t23100 / 16.0_f64 - t23102 / 16.0_f64 - t23105 / 32.0_f64 - t23108 / 32.0_f64 - t23111 / 48.0_f64 + t23113 / 24.0_f64 - 5.0_f64 / 144.0_f64 * t23116 - t23118 / 16.0_f64 - t23120 / 32.0_f64 - 5.0_f64 / 432.0_f64 * t20022 - 3.0_f64 / 16.0_f64 * t7831 * t9858 - 5.0_f64 / 144.0_f64 * t23124 - 41.0_f64 / 48.0_f64 * t23128 - t23139 / 36.0_f64 + t20070 / 144.0_f64 + t20073 / 144.0_f64 + t23572 / 24.0_f64 + t23575 / 24.0_f64;
    t27706
}
