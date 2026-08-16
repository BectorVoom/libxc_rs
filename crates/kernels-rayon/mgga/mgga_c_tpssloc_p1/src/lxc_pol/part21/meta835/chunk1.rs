//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2963/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2963(t17884: f64, t3117: f64, t18029: f64, t3108: f64, t1021: f64, t1025: f64, t10863: f64, t10957: f64, t10965: f64, t1618: f64, t17607: f64, t248: f64, t3043: f64, t3057: f64, t3064: f64, t3098: f64, t3130: f64, t3131: f64, t3134: f64, t48446: f64, t49678: f64, t5857: f64, t5861: f64, t5900: f64, t61719: f64, t61731: f64, t61736: f64, t61739: f64, t61742: f64) -> f64 {
    let t61744 = t3117 * t17884;
    let t61754 = t18029 * t3108;
    let t61760 = t3130 * t248 * t1021 * t61719 * t3131 / 768.0_f64 + t17607 * t3057 / 4608.0_f64 + 5.0_f64 / 13824.0_f64 * t17607 * t3064 + t61731 * t1025 / 1536.0_f64 + t61736 * t3134 / 1536.0_f64 - t61739 * t3043 / 3072.0_f64 + t61742 / 432.0_f64 + 5.0_f64 / 10368.0_f64 * t61744 + t10965 * t5857 / 4608.0_f64 + 19.0_f64 / 864.0_f64 * t49678 * t1618 + t10863 * t5900 / 216.0_f64 + 95.0_f64 / 7776.0_f64 * t10957 * t5861 - t61754 * t1025 / 288.0_f64 - t17607 * t3098 / 2304.0_f64 + 19.0_f64 / 1296.0_f64 * t48446;
    t61760
}
