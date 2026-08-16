//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2565/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2565(t11185: f64, t21724: f64, t11297: f64, t11365: f64, t1138: f64, t11415: f64, t1155: f64, t1157: f64, t15146: f64, t1695: f64, t18637: f64, t18644: f64, t18785: f64, t21836: f64, t21947: f64, t21952: f64, t3376: f64, t3401: f64, t4857: f64, t4858: f64, t51427: f64, t51730: f64, t6037: f64, t6069: f64, t6084: f64, t71850: f64, t71853: f64, t71855: f64, t71860: f64, t71863: f64) -> (f64, f64) {
    let t71867 = 6.0_f64 * t11185 * t21724;
    let t71868 = 0.96491876992155210402e2_f64 * t15146 * t18644 - 0.57895126195293126241e3_f64 * t51427 * t18637 - 0.14035736694323150897e2_f64 * t11365 * t21947 * t1155 + 0.10526802520742363173e2_f64 * t3401 * t6069 * t4857 - 0.35089341735807877242e1_f64 * t11297 * t21836 - 0.35089341735807877242e1_f64 * t3376 * t4858 * t6084 - 0.35089341735807877242e1_f64 * t3376 * t1695 * t18785 + t71850 - t71853 + t71855 - 6.0_f64 * t51730 * t6037 + 6.0_f64 * t11415 * t21952 + 0.5848223622634646207e0_f64 * t71860 * t1157 + 1.0_f64 * t71863 * t1138 - t71867;
    (t71867, t71868)
}
