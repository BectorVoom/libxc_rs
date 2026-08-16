//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2799/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2799(t12984: f64, t12998: f64, t4119: f64, t686: f64, t12971: f64, t13005: f64, t16771: f64, t16796: f64, t221: f64, t2379: f64, t2553: f64, t4127: f64, t4128: f64, t46770: f64, t46772: f64, t46780: f64, t46847: f64, t59138: f64, t59140: f64, t59154: f64, t59156: f64, t59165: f64) -> f64 {
    let t59173 = t12998 * t686 * t12984 * t4119;
    let t59178 = -0.49999999999999999998e-2_f64 * t59138 - 0.23333333333333333332e-1_f64 * t59140 + 0.49999999999999999998e-2_f64 * t4127 * t221 * t16796 * t2553 - 0.19999999999999999999e-1_f64 * t13005 * t221 * t16796 * t2379 + 0.99999999999999999995e-1_f64 * t46847 * t221 * t16771 * t2379 + 0.93333333333333333328e-1_f64 * t59154 - 0.46666666666666666664e-1_f64 * t59156 - 0.19999999999999999999e-1_f64 * t13005 * t221 * t16771 * t2553 + 0.19999999999999999999e-1_f64 * t59165 + 0.99999999999999999996e-2_f64 * t4127 * t221 * t4128 * t12971 - 0.99999999999999999996e-2_f64 * t59173 - 0.5185185185185185185e-1_f64 * t46770 + 0.65740740740740740737e-1_f64 * t46772 + 0.16666666666666666666e-2_f64 * t46780;
    t59178
}
