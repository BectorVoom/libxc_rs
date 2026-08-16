//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2269/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2269(t12971: f64, t13283: f64, t13300: f64, t1484: f64, t1512: f64, t2553: f64, t2643: f64, t2645: f64, t2684: f64, t2701: f64, t4119: f64, t41399: f64, t4236: f64, t46952: f64, t46954: f64, t46957: f64, t46960: f64, t46962: f64, t46974: f64, t46980: f64, t776: f64, t820: f64, t843: f64, t9516: f64, t9613: f64, t9978: f64, t9983: f64) -> f64 {
    let t46982 = -t41399 * t1512 / 3072.0_f64 - t9613 * t4236 / 1024.0_f64 + 5.0_f64 / 768.0_f64 * t843 * t2701 * t820 * t1484 * t9516 - t46952 - t46954 + t13283 * t9983 / 512.0_f64 - t46957 * t9978 / 512.0_f64 + 7.0_f64 / 1536.0_f64 * t46960 - 35.0_f64 / 384.0_f64 * t46962 + 5.0_f64 / 256.0_f64 * t843 * t2701 * t820 * t12971 * t776 + 5.0_f64 / 256.0_f64 * t843 * t2701 * t820 * t4119 * t2553 - 7.0_f64 / 192.0_f64 * t46974 + t2643 * t2645 * t13300 * t2684 / 256.0_f64 - 7.0_f64 / 384.0_f64 * t46980;
    t46982
}
