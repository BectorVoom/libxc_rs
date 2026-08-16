//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2979/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2979(t10508: f64, t248: f64, t3039: f64, t5878: f64, t1041: f64, t10863: f64, t13980: f64, t14085: f64, t14107: f64, t14180: f64, t17693: f64, t17712: f64, t3117: f64, t3130: f64, t4582: f64, t4585: f64, t4644: f64, t49734: f64, t49748: f64, t49854: f64, t50193: f64, t5861: f64, t61855: f64, t62148: f64, t62150: f64, t62152: f64, t62164: f64, t62177: f64) -> f64 {
    let t62183 = t3039 * t248 * t10508 * t5878;
    let t62185 = -t62148 / 6912.0_f64 - t62150 / 648.0_f64 + t62152 / 1152.0_f64 + t49734 / 2304.0_f64 + t3130 * t4582 * t17712 * t13980 / 1536.0_f64 + 5.0_f64 / 3456.0_f64 * t4644 * t14180 + t50193 * t14107 / 1536.0_f64 - t62164 / 2304.0_f64 - 5.0_f64 / 432.0_f64 * t1041 * t4582 * t49854 * t61855 + 5.0_f64 / 3456.0_f64 * t3117 * t17693 - t14085 * t4585 / 576.0_f64 + 5.0_f64 / 3888.0_f64 * t49748 - t62177 / 13824.0_f64 - 5.0_f64 / 1296.0_f64 * t10863 * t5861 + t62183 / 13824.0_f64;
    t62185
}
