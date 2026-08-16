//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3178/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3178(t13969: f64, t19061: f64, t3515: f64, t11665: f64, t11668: f64, t11678: f64, t11692: f64, t1227: f64, t14731: f64, t14736: f64, t14740: f64, t15654: f64, t1735: f64, t19016: f64, t19068: f64, t3490: f64, t3509: f64, t3516: f64, t3577: f64, t3578: f64, t4582: f64, t4724: f64, t4987: f64, t5012: f64, t52725: f64, t52731: f64, t52733: f64, t55662: f64, t55666: f64, t5979: f64, t62044: f64) -> f64 {
    let t65881 = t3515 * t13969 * t19061;
    let t65883 = 5.0_f64 / 3456.0_f64 * t11665 * t19016 + 5.0_f64 / 3456.0_f64 * t3577 * t11668 * t5012 * t4724 + 5.0_f64 / 3456.0_f64 * t3577 * t11668 * t1735 * t14736 + 5.0_f64 / 6912.0_f64 * t3577 * t11668 * t1735 * t14740 + 5.0_f64 / 1152.0_f64 * t3577 * t11668 * t1735 * t14731 - t11678 * t3578 * t5979 * t3509 / 2304.0_f64 + t11692 * t3578 * t5979 * t3516 / 4608.0_f64 + 5.0_f64 / 10368.0_f64 * t52725 - t52731 / 3456.0_f64 - t52733 / 1728.0_f64 + 5.0_f64 / 6912.0_f64 * t3490 * t19068 + 5.0_f64 / 6912.0_f64 * t1227 * t4582 * t4987 * t55666 + 5.0_f64 / 13824.0_f64 * t1227 * t4582 * t4987 * t55662 + 5.0_f64 / 2304.0_f64 * t1227 * t4582 * t15654 * t62044 - t65881 / 2304.0_f64;
    t65883
}
