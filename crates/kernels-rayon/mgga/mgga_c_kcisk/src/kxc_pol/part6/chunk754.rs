//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 754/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk754(t120: f64, t15418: f64, t121: f64, t129: f64, t15232: f64, t15237: f64, t15245: f64, t15255: f64, t15259: f64, t15262: f64, t15270: f64, t3033: f64, t3036: f64, t3044: f64, t3054: f64, t3060: f64, t913: f64, t920: f64, t929: f64) -> f64 {
    let t15419 = t15418 * t120;
    let t15422 = -0.75561312607944732299e0_f64 * t920 * t3054 + 0.32383419689119170984e0_f64 * t913 * t3054 + 0.1259355210132412205e1_f64 * t15232 * t129 + 0.75561312607944732299e0_f64 * t3036 * t929 - 0.3778065630397236615e0_f64 * t15237 * t129 - 0.16191709844559585492e0_f64 * t3033 * t929 - 0.16191709844559585492e0_f64 * t913 * t3060 - 0.18190686368579287406e1_f64 * t15245 * t129 - 0.1259355210132412205e1_f64 * t3044 * t929 - 0.32383419689119170984e0_f64 * t121 * t15255 + 0.32383419689119170984e0_f64 * t15259 * t15262 + 0.3778065630397236615e0_f64 * t920 * t3060 - 0.53972366148531951642e-1_f64 * t121 * t15270 + 0.53972366148531951642e-1_f64 * t15419 * t129;
    t15422
}
