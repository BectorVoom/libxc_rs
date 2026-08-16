//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2483/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2483(t1041: f64, t13969: f64, t21550: f64, t1023: f64, t10937: f64, t14218: f64, t17697: f64, t21570: f64, t2986: f64, t42358: f64, t43361: f64, t4582: f64, t4644: f64, t48611: f64, t49907: f64, t49923: f64, t50366: f64, t62343: f64, t62349: f64, t62360: f64, t62840: f64, t68513: f64, t70273: f64) -> f64 {
    let t70640 = t1041 * t13969 * t21550;
    let t70645 = -3.0_f64 / 512.0_f64 * t43361 * t48611 * t62840 * t14218 + t49907 + t62343 / 1536.0_f64 - t62349 / 768.0_f64 - t49923 - t62360 / 4608.0_f64 + 5.0_f64 / 1728.0_f64 * t4644 * t17697 - 5.0_f64 / 864.0_f64 * t10937 * t21570 - t42358 * t4582 * t70273 * t1023 / 3072.0_f64 - t70640 / 1152.0_f64 + t2986 * t50366 * t68513 / 16.0_f64;
    t70645
}
