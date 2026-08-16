//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2036/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2036<F: Float>(t118: F, t159: F, t168: F, t2458: F, t2459: F, t2461: F, t2471: F, t2472: F, t2475: F, t2476: F, t2479: F, t2495: F, t2504: F, t2510: F, t2512: F, t39273: F, t39275: F, t39278: F, t39281: F, t39283: F, t39284: F, t39289: F, t39291: F, t39293: F, t39295: F, t39298: F, t39378: F, t39389: F, t39463: F, t39468: F, t39472: F, t39476: F, t39483: F, t39664: F, t690: F, t725: F, t730: F, t731: F, t9730: F, t9733: F, t9739: F, t9758: F, t9892: F, t9905: F) -> F {
    let t39803 = F::cast_from(0.12414243100625616072e5_f64) * t9730 * t2471 * t9733 * t2461 + F::cast_from(0.1301229756036208781e0_f64) * t690 * t9905 - F::cast_from(0.24828486201251232145e5_f64) * t159 / t2475 / t2458 * t39664 * t9733 + F::cast_from(1.0_f64) * t725 * (-F::cast_from(0.39219166666666666667e1_f64) * t39273 + F::cast_from(0.376504e2_f64) * t39275 - F::cast_from(0.13944592592592592593e2_f64) * t39278 + F::cast_from(0.12201518518518518519e2_f64) * t39281 + F::cast_from(0.5356037037037037037e1_f64) * t39284 + F::cast_from(0.14025833333333333333e0_f64) * t39289 - F::cast_from(0.22441333333333333332e1_f64) * t39291 + F::cast_from(0.24934814814814814815e1_f64) * t39293 + F::cast_from(0.21817962962962962963e1_f64) * t39295 + F::cast_from(0.16979925925925925926e1_f64) * t39298) * t731 + F::cast_from(0.21053605041484726346e2_f64) * t2510 * t2495 * t2504 - t39463 + t39468 + F::cast_from(0.51947577317044391277e2_f64) * t2510 * t39389 * t2512 + t39472 + t39476 - F::cast_from(24.0_f64) * t9739 * t39664 * t731 - t39483 - F::cast_from(0.55209406483950617283e-2_f64) * t118 * t39283 * t168 + F::cast_from(0.6233709278045326953e3_f64) * t9758 * t39378 * t2512 + F::cast_from(0.41096e0_f64) * t690 * t2459 * t730 * t2472 - F::cast_from(0.6609050294782684211e1_f64) * t690 * t2476 * t2471 * t2479 * t730 - F::cast_from(0.19263893255070628431e1_f64) * t690 * t9892;
    t39803
}
