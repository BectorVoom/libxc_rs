//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2972/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2972(t10952: f64, t17655: f64, t17659: f64, t3117: f64, t1041: f64, t17187: f64, t248: f64, t3051: f64, t10390: f64, t10480: f64, t10904: f64, t13762: f64, t13995: f64, t14488: f64, t17670: f64, t17714: f64, t17998: f64, t3040: f64, t3071: f64, t3130: f64, t3131: f64, t42552: f64, t42573: f64, t43291: f64, t43292: f64, t4582: f64, t4593: f64, t4596: f64, t48607: f64, t49651: f64, t49682: f64, t49684: f64, t50510: f64, t5880: f64, t61078: f64) -> f64 {
    let t61975 = t10952 * t17655;
    let t61977 = t3117 * t17659;
    let t61981 = t1041 * t248 * t3051 * t17187;
    let t62007 = 5.0_f64 / 6912.0_f64 * t10390 * t17998 + t13995 * t13762 / 1152.0_f64 + t42573 * t5880 / 288.0_f64 - t61975 / 2304.0_f64 + t61977 / 3456.0_f64 + t61981 / 3456.0_f64 + t10480 * t4582 * t17670 * t50510 / 512.0_f64 + t43291 * t4582 * t17670 * t43292 * t3040 / 128.0_f64 + t3130 * t4582 * t4593 * t3131 * t14488 / 768.0_f64 + 5.0_f64 / 1944.0_f64 * t42552 - t10904 * t17714 / 144.0_f64 + t49682 / 1728.0_f64 + t48607 * t3071 * t61078 / 192.0_f64 + 2.0_f64 / 81.0_f64 * t49684 + t49651 * t4596 / 384.0_f64;
    t62007
}
