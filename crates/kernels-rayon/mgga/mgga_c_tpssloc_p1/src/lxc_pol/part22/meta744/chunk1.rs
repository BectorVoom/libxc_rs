//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2469/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2469(t21390: f64, t376: f64, t10952: f64, t1616: f64, t17607: f64, t17712: f64, t21503: f64, t21551: f64, t3039: f64, t3048: f64, t3117: f64, t42347: f64, t4582: f64, t4585: f64, t4590: f64, t4594: f64, t4650: f64, t61784: f64, t61794: f64, t61796: f64, t62091: f64) -> (f64, f64) {
    let t70273 = t376 * t21390;
    let t70296 = -t61784 / 576.0_f64 - t3117 * t21551 / 768.0_f64 + 7.0_f64 / 1536.0_f64 * t42347 * t4582 * t70273 * t4594 + t61794 / 768.0_f64 - t17607 * t4585 / 768.0_f64 + 5.0_f64 / 4608.0_f64 * t17607 * t4590 + 5.0_f64 / 3456.0_f64 * t61796 - t10952 * t21503 / 1024.0_f64 - t3039 * t4582 * t62091 * t1616 / 1024.0_f64 - t3039 * t4582 * t17712 * t4650 / 1024.0_f64 + t3048 * t21551 / 144.0_f64;
    (t70273, t70296)
}
