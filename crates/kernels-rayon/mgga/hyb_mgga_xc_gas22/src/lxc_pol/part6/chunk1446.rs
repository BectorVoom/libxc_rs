//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1446/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1446(t11376: f64, t3957: f64, t9645: f64, t9656: f64, t101: f64, t11391: f64, t9522: f64, t9534: f64, t11346: f64, t13638: f64, t13643: f64, t26226: f64, t26231: f64, t26403: f64, t26416: f64, t26425: f64, t26654: f64, t31055: f64, t31058: f64, t31352: f64, t31355: f64, t4491: f64, t9663: f64, t9667: f64, t9670: f64, t9678: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31479 = t11376 * t3957;
    let t31480 = t31479 * t9645;
    let t31483 = t31479 * t9656;
    let t31492 = t11391 * t101;
    let t31493 = t31492 * t9522;
    let t31496 = t31492 * t9534;
    let t31501 = t11346 * t13638;
    let t31504 = t11346 * t13643;
    let t31511 = -256.0_f64 / 27.0_f64 * t9663 * t31480 + 256.0_f64 / 27.0_f64 * t9670 * t31483 + 2048.0_f64 / 729.0_f64 * t26231 * t31355 - 512.0_f64 / 81.0_f64 * t9678 * t31055 + 512.0_f64 / 81.0_f64 * t9667 * t31058 - 2000.0_f64 * t26403 * t31493 + 2800.0_f64 * t26416 * t31496 - 2048.0_f64 / 729.0_f64 * t26226 * t31352 + 2000.0_f64 * t26403 * t31501 - 2800.0_f64 * t26416 * t31504 - 4000.0_f64 / 3.0_f64 * t26425 * t31493 - 16.0_f64 / 9.0_f64 * t26654 * t4491;
    (t31480, t31483, t31496, t31501, t31504, t31511)
}
