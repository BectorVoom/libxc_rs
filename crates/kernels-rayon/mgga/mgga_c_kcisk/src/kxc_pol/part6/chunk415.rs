//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 415/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk415(t852: f64, t855: f64, t135: f64, t854: f64, t60: f64, t932: f64, t132: f64, t68: f64, t69: f64, t142: f64, t862: f64, t85: f64, t861: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2927 = t852 * t855;
    let t2931 = 1.0_f64 / t854 / t135;
    let t2932 = t60 * t2931;
    let t2933 = t932 * t932;
    let t2934 = t132 * t132;
    let t2935 = 1.0_f64 / t2934;
    let t2936 = t2933 * t2935;
    let t2942 = t68 * t69;
    let t2943 = t142 * t862;
    let t2947 = t861 * t85;
    (t2927, t2931, t2932, t2933, t2934, t2935, t2936, t2942, t2943, t2947)
}
