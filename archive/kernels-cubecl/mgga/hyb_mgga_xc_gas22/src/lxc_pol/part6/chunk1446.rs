//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1446/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1446<F: Float>(t11376: F, t3957: F, t9645: F, t9656: F, t101: F, t11391: F, t9522: F, t9534: F, t11346: F, t13638: F, t13643: F, t26226: F, t26231: F, t26403: F, t26416: F, t26425: F, t26654: F, t31055: F, t31058: F, t31352: F, t31355: F, t4491: F, t9663: F, t9667: F, t9670: F, t9678: F) -> (F, F, F, F, F, F) {
    let t31479 = t11376 * t3957;
    let t31480 = t31479 * t9645;
    let t31483 = t31479 * t9656;
    let t31492 = t11391 * t101;
    let t31493 = t31492 * t9522;
    let t31496 = t31492 * t9534;
    let t31501 = t11346 * t13638;
    let t31504 = t11346 * t13643;
    let t31511 = -F::cast_from(256.0_f64) / F::cast_from(27.0_f64) * t9663 * t31480 + F::cast_from(256.0_f64) / F::cast_from(27.0_f64) * t9670 * t31483 + F::cast_from(2048.0_f64) / F::cast_from(729.0_f64) * t26231 * t31355 - F::cast_from(512.0_f64) / F::cast_from(81.0_f64) * t9678 * t31055 + F::cast_from(512.0_f64) / F::cast_from(81.0_f64) * t9667 * t31058 - F::cast_from(2000.0_f64) * t26403 * t31493 + F::cast_from(2800.0_f64) * t26416 * t31496 - F::cast_from(2048.0_f64) / F::cast_from(729.0_f64) * t26226 * t31352 + F::cast_from(2000.0_f64) * t26403 * t31501 - F::cast_from(2800.0_f64) * t26416 * t31504 - F::cast_from(4000.0_f64) / F::cast_from(3.0_f64) * t26425 * t31493 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t26654 * t4491;
    (t31480, t31483, t31496, t31501, t31504, t31511)
}
