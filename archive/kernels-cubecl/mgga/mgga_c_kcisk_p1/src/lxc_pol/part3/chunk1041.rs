//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 1041/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk1041<F: Float>(t120: F, t15418: F, t121: F, t129: F, t15232: F, t15237: F, t15245: F, t15255: F, t15259: F, t15262: F, t15270: F, t3033: F, t3036: F, t3044: F, t3054: F, t3060: F, t913: F, t920: F, t929: F) -> F {
    let t15419 = t15418 * t120;
    let t15422 = -F::cast_from(0.75561312607944732299e0_f64) * t920 * t3054 + F::cast_from(0.32383419689119170984e0_f64) * t913 * t3054 + F::cast_from(0.1259355210132412205e1_f64) * t15232 * t129 + F::cast_from(0.75561312607944732299e0_f64) * t3036 * t929 - F::cast_from(0.3778065630397236615e0_f64) * t15237 * t129 - F::cast_from(0.16191709844559585492e0_f64) * t3033 * t929 - F::cast_from(0.16191709844559585492e0_f64) * t913 * t3060 - F::cast_from(0.18190686368579287406e1_f64) * t15245 * t129 - F::cast_from(0.1259355210132412205e1_f64) * t3044 * t929 - F::cast_from(0.32383419689119170984e0_f64) * t121 * t15255 + F::cast_from(0.32383419689119170984e0_f64) * t15259 * t15262 + F::cast_from(0.3778065630397236615e0_f64) * t920 * t3060 - F::cast_from(0.53972366148531951642e-1_f64) * t121 * t15270 + F::cast_from(0.53972366148531951642e-1_f64) * t15419 * t129;
    t15422
}
