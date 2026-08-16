//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 217/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk217(t60: f64, t116: f64, t918: f64, t114: f64, t126: f64, t6: f64, t852: f64, t123: f64, t121: f64, t129: f64, t913: f64, t132: f64, t119: f64, t177: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t124 = 0.0_f64 < t60;
    let t919 = t116 * t918;
    let t920 = t114 * t919;
    let t923 = t126 * t126;
    let t924 = 1.0_f64 / t923;
    let t925 = t6 * t924;
    let t927 = piecewise3(t124, t852, -t852);
    let t929 = t123 * t925 * t927;
    let t932 = 0.53972366148531951642e-1_f64 * t913 * t129 - 0.1259355210132412205e0_f64 * t920 * t129 - 0.53972366148531951642e-1_f64 * t121 * t929;
    let t933 = 1.0_f64 / t132;
    let t934 = t932 * t933;
    let t937 = t119 * t177;
    (t919, t920, t923, t925, t927, t929, t932, t933, t934, t937)
}
