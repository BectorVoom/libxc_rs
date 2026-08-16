//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2467/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2467(t1041: f64, t21134: f64, t248: f64, t3051: f64, t14508: f64, t17667: f64, t14085: f64, t1622: f64, t17962: f64, t21405: f64, t21580: f64, t21597: f64, t3109: f64, t3117: f64, t42354: f64, t4641: f64, t48431: f64, t50302: f64, t5857: f64, t5875: f64, t61677: f64, t61695: f64) -> f64 {
    let t70199 = t1041 * t248 * t3051 * t21134;
    let t70209 = t14508 * t17667;
    let t70211 = -t61695 / 288.0_f64 + t48431 - t3109 * t21597 / 576.0_f64 + t4641 * t17962 / 1024.0_f64 + t14085 * t5857 / 1536.0_f64 + t70199 / 6912.0_f64 - 5.0_f64 / 2304.0_f64 * t3117 * t21580 + t42354 * t21405 / 3072.0_f64 + t61677 * t1622 / 1536.0_f64 - t50302 * t5875 / 96.0_f64 + t70209 / 768.0_f64;
    t70211
}
