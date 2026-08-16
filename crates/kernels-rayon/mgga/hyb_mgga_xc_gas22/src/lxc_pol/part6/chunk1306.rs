//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1306/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1306(t143: f64, t28311: f64, t3227: f64, t3246: f64, t4046: f64, t6394: f64, t10350: f64, t10411: f64, t173: f64, t178: f64, t180: f64, t181: f64, t2111: f64, t2124: f64, t2132: f64, t3255: f64, t4068: f64, t747: f64, t751: f64, t8354: f64, t8395: f64, t8396: f64, t8402: f64, t8415: f64, t8418: f64) -> (f64, f64, f64, f64) {
    let t145 = 0.135e1_f64 < t143;
    let t28459 = piecewise3(t145, 0.0_f64, t28311);
    let t28476 = t3246 * t3227;
    let t28505 = t3227 * t3227;
    let t28530 = t6394 * t4046;
    let t28538 = 30.0_f64 * t8395 * t28476 - 10.0_f64 * t8402 * t28476 + t8415 * t28476 / 2.0_f64 - 8.0_f64 * t28505 * t181 + t747 * t28459 * t180 / 2.0_f64 - 8.0_f64 * t10411 * t8354 - 4.0_f64 * t8418 * t4046 - 8.0_f64 * t3255 * t10350 - 4.0_f64 * t751 * t28459 - t173 * t28459 * t180 - 2.0_f64 * t178 * t28505 * t180 - 4.0_f64 * t2124 * t28505 * t180 + t2132 * t28505 * t180 / 2.0_f64 + t28530 * t8396 / 8.0_f64 - 75.0_f64 / 2.0_f64 * t4068 * t8396 + 15.0_f64 / 2.0_f64 * t2111 * t4046 * t8396;
    (t28459, t28476, t28505, t28538)
}
