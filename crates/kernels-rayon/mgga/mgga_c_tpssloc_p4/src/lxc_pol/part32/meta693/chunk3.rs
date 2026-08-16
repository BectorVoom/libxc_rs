//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2149/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2149(t1992: f64, t20018: f64, t6976: f64, t550: f64, t57499: f64, t22704: f64, t22705: f64, t28163: f64, t57618: f64, t1332: f64, t19805: f64, t2013: f64, t28156: f64, t81061: f64, t81066: f64, t81073: f64, t81075: f64, t81076: f64, t90899: f64, t90913: f64, t93563: f64, t97002: f64, t97007: f64, t97014: f64) -> f64 {
    let t97017 = t1992 * t6976 * t20018;
    let t97023 = t1992 * t6976 * t57499 * t550;
    let t97026 = t22704 * t22705 * t28163;
    let t97030 = t1992 * t6976 * t57618 * t550;
    let t97032 = -0.49348022005446793095e-1_f64 * t97002 - 0.63969658155208805863e-1_f64 * t81061 - 0.3289868133696452873e-1_f64 * t97007 - t90899 + t1332 * t28156 + t93563 + 0.82246703342411321824e-2_f64 * t81066 - t90913 - 0.19739208802178717238e0_f64 * t97014 - 0.16449340668482264365e-1_f64 * t97017 - t81073 - t81075 + 0.26044789391763585244e-1_f64 * t81076 + t19805 * t2013 - 0.16449340668482264365e-1_f64 * t97023 + 0.82246703342411321825e-2_f64 * t97026 - 0.82246703342411321825e-2_f64 * t97030;
    t97032
}
