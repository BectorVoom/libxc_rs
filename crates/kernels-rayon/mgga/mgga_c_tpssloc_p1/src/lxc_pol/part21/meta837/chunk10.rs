//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2988/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2988(t3070: f64, t43198: f64, t5908: f64, t10937: f64, t18041: f64, t1041: f64, t13969: f64, t17636: f64, t10408: f64, t10413: f64, t10952: f64, t13528: f64, t14143: f64, t14147: f64, t14184: f64, t14489: f64, t1616: f64, t1622: f64, t17718: f64, t17738: f64, t2776: f64, t2960: f64, t3039: f64, t3071: f64, t43358: f64, t4582: f64, t4593: f64, t4644: f64, t48432: f64, t50047: f64, t50056: f64, t5878: f64, t5909: f64) -> f64 {
    let t62494 = t3070 * t43198 * t5908;
    let t62499 = t10937 * t18041;
    let t62510 = t1041 * t13969 * t17636;
    let t62512 = -t4644 * t14143 / 576.0_f64 - t4644 * t14147 / 1152.0_f64 - t2960 * t17738 / 54.0_f64 + 5.0_f64 / 6912.0_f64 * t4644 * t14184 + 5.0_f64 / 3456.0_f64 * t3070 * t10408 * t1616 * t13528 + t50047 / 2592.0_f64 + t10413 * t3071 * t5878 * t2776 / 2304.0_f64 - t62494 / 10368.0_f64 + t50056 / 3456.0_f64 + 19.0_f64 / 1296.0_f64 * t43358 * t5909 - t62499 / 324.0_f64 + t48432 * t1622 / 2304.0_f64 - t10952 * t17718 / 1536.0_f64 - t3039 * t4582 * t4593 * t14489 / 1536.0_f64 - t62510 / 1728.0_f64;
    t62512
}
