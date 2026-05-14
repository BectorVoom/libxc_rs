//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 921/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk921<F: Float>(t110: F, t15363: F, t15401: F, t67: F, t10: F, t107: F, t119: F, t142: F, t3020: F, t64: F, t903: F, t918: F, t41: F, t120: F, t121: F, t129: F, t15232: F, t15237: F, t15245: F, t15255: F, t15259: F, t15262: F, t15270: F, t3033: F, t3036: F, t3044: F, t3054: F, t3060: F, t913: F, t920: F, t929: F) -> (F,) {
    let t111 = t110 < -0.66725e-1;
    let t15403 = t67 * (t15363 + t15401);
    let t15417 = piecewise3(t111, 0.0, 10.0 / 9.0 * t64 * t15403 * t10 - 10.0 / 9.0 * t64 * t3020 * t142 + 40.0 / 27.0 * t64 * t903 * t119 - 280.0 / 243.0 * t64 * t107 * t918);
    let t15418 = t15417 * t41;
    let t15419 = t15418 * t120;
    let t15422 = -0.75561312607944732299e0 * t920 * t3054 + 0.32383419689119170984e0 * t913 * t3054 + 0.1259355210132412205e1 * t15232 * t129 + 0.75561312607944732299e0 * t3036 * t929 - 0.3778065630397236615e0 * t15237 * t129 - 0.16191709844559585492e0 * t3033 * t929 - 0.16191709844559585492e0 * t913 * t3060 - 0.18190686368579287406e1 * t15245 * t129 - 0.1259355210132412205e1 * t3044 * t929 - 0.32383419689119170984e0 * t121 * t15255 + 0.32383419689119170984e0 * t15259 * t15262 + 0.3778065630397236615e0 * t920 * t3060 - 0.53972366148531951642e-1 * t121 * t15270 + 0.53972366148531951642e-1 * t15419 * t129;
    (t15422,)
}
