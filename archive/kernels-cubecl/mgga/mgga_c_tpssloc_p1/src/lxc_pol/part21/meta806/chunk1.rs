//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2799/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2799<F: Float>(t12984: F, t12998: F, t4119: F, t686: F, t12971: F, t13005: F, t16771: F, t16796: F, t221: F, t2379: F, t2553: F, t4127: F, t4128: F, t46770: F, t46772: F, t46780: F, t46847: F, t59138: F, t59140: F, t59154: F, t59156: F, t59165: F) -> F {
    let t59173 = t12998 * t686 * t12984 * t4119;
    let t59178 = -F::cast_from(0.49999999999999999998e-2_f64) * t59138 - F::cast_from(0.23333333333333333332e-1_f64) * t59140 + F::cast_from(0.49999999999999999998e-2_f64) * t4127 * t221 * t16796 * t2553 - F::cast_from(0.19999999999999999999e-1_f64) * t13005 * t221 * t16796 * t2379 + F::cast_from(0.99999999999999999995e-1_f64) * t46847 * t221 * t16771 * t2379 + F::cast_from(0.93333333333333333328e-1_f64) * t59154 - F::cast_from(0.46666666666666666664e-1_f64) * t59156 - F::cast_from(0.19999999999999999999e-1_f64) * t13005 * t221 * t16771 * t2553 + F::cast_from(0.19999999999999999999e-1_f64) * t59165 + F::cast_from(0.99999999999999999996e-2_f64) * t4127 * t221 * t4128 * t12971 - F::cast_from(0.99999999999999999996e-2_f64) * t59173 - F::cast_from(0.5185185185185185185e-1_f64) * t46770 + F::cast_from(0.65740740740740740737e-1_f64) * t46772 + F::cast_from(0.16666666666666666666e-2_f64) * t46780;
    t59178
}
