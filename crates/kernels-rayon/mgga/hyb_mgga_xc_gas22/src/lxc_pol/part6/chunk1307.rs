//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1307/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1307(t10350: f64, t180: f64, t2132: f64, t10364: f64, t10373: f64, t10394: f64, t10397: f64, t10403: f64, t10408: f64, t10414: f64, t20467: f64, t20475: f64, t2124: f64, t24354: f64, t3245: f64, t3246: f64, t3252: f64, t3258: f64, t4046: f64, t4051: f64, t4052: f64, t6355: f64, t6383: f64, t8396: f64, t8423: f64) -> (f64, f64) {
    let t28549 = t180 * t10350;
    let t28571 = t2132 * t10350;
    let t28576 = t3252 * t24354 / 2.0_f64 + t10408 * t6355 / 8.0_f64 + t20475 * t4051 * t8396 / 16.0_f64 - 2.0_f64 * t10414 * t24354 - t8423 * t10373 - 2.0_f64 * t3258 * t28549 + 15.0_f64 / 2.0_f64 * t4052 * t6355 + 85.0_f64 / 4.0_f64 * t10364 * t8396 - 4.0_f64 * t3245 * t24354 - 5.0_f64 / 2.0_f64 * t10394 * t6355 - 19.0_f64 / 8.0_f64 * t20467 * t4051 * t8396 - 4.0_f64 * t2124 * t10350 * t3246 - 2.0_f64 * t10397 * t6355 - 5.0_f64 / 2.0_f64 * t6383 * t4046 * t8396 + t28571 * t3246 / 2.0_f64 + t10403 * t6355 / 4.0_f64;
    (t28549, t28576)
}
