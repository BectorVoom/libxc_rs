//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1731/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1731(t1164: f64, t18926: f64, t4869: f64, t4875: f64, t18711: f64, t300: f64, t3375: f64, t6084: f64, t1157: f64, t3411: f64, t6102: f64, t18682: f64, t18685: f64, t18688: f64, t18690: f64, t18692: f64, t18694: f64, t18696: f64, t18837: f64, t18839: f64, t18917: f64, t18920: f64, t18922: f64, t18924: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18928 = 0.5848223622634646207e0_f64 * t1164 * t18926;
    let t18930 = 0.23392894490538584828e1_f64 * t4869 * t4875;
    let t18932 = 0.19751673498613801407e-1_f64 * t300 * t18711;
    let t18933 = t3375 * t6084;
    let t18934 = t18933 * t1157;
    let t18936 = 0.11696447245269292414e1_f64 * t1164 * t18934;
    let t18938 = 0.5848223622634646207e0_f64 * t3411 * t6102;
    let t18939 = -t18682 - t18685 - t18917 + t18920 + t18922 - t18924 + t18688 + t18690 + t18692 - t18694 + t18696 - t18928 + t18930 + t18932 + t18837 + t18839 + t18936 - t18938;
    (t18928, t18930, t18932, t18934, t18936, t18938, t18939)
}
