//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1773/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1773(t81230: f64, t81232: f64, t81234: f64, t81425: f64, t81427: f64, t81429: f64, t89828: f64, t89843: f64, t89847: f64, t89855: f64, t90459: f64, t90464: f64, t90470: f64, t90473: f64) -> f64 {
    let t90717 = -0.123954e2_f64 * t89828 + 0.3529725e1_f64 * t90459 - 0.27785333333333333333e0_f64 * t81425 + 0.55570666666666666668e0_f64 * t81427 - 0.166712e1_f64 * t81429 + 0.94674375e0_f64 * t90464 - 0.13772666666666666667e1_f64 * t89843 + 0.185931e2_f64 * t89847 + 0.41318e1_f64 * t89855 - 0.13892666666666666667e0_f64 * t90470 - 0.125034e1_f64 * t90473 - 0.76514814814814814814e0_f64 * t81230 + 0.27545333333333333332e1_f64 * t81232 - 0.41318e1_f64 * t81234;
    t90717
}
