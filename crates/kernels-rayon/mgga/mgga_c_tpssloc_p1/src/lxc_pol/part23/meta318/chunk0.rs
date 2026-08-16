//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1077/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1077(t21749: f64, t4908: f64, t18420: f64, t4904: f64, t20246: f64, t338: f64, t11556: f64, t15300: f64, t15364: f64, t15376: f64, t18447: f64, t18452: f64, t18455: f64, t18458: f64, t18460: f64, t18489: f64, t18530: f64, t18533: f64, t18536: f64, t3447: f64, t463: f64, t4889: f64, t6123: f64, t6127: f64, t6131: f64) -> (f64, f64, f64, f64) {
    let t22090 = t4908 * t21749;
    let t22095 = t18420 * t4904;
    let t22104 = t20246 * t338;
    let t22112 = 0.22222222222222222221e-2_f64 * t4889 * t6131 + 0.44444444444444444442e-2_f64 * t4889 * t6127 - 0.16666666666666666666e-2_f64 * t3447 * t22090 - 0.44444444444444444443e-2_f64 * t15376 * t6123 + 0.83333333333333333331e-3_f64 * t3447 * t22095 + 0.55555555555555555554e-3_f64 * t18447 - 0.55555555555555555554e-3_f64 * t18452 - 0.27777777777777777777e-3_f64 * t18455 + 0.37037037037037037036e-3_f64 * t18458 + 0.14814814814814814814e-2_f64 * t18460 + 0.18518518518518518518e-3_f64 * t15300 - 0.38024691358024691358e-1_f64 * t22104 * t463 + 0.55555555555555555554e-3_f64 * t15364 + 0.81481481481481481478e-2_f64 * t18489 - 0.83333333333333333331e-3_f64 * t18530 - 0.83333333333333333331e-3_f64 * t18533 + 0.44444444444444444443e-2_f64 * t18536 + t11556;
    (t22090, t22095, t22104, t22112)
}
