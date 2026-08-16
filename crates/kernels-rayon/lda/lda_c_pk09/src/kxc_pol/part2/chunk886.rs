//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 886/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk886(t9291: f64, t9301: f64, t9313: f64, t9328: f64, t83: f64, t89: f64, t1052: f64, t1059: f64, t3317: f64, t3319: f64, t3323: f64, t3326: f64, t4398: f64, t4406: f64, t8892: f64, t8895: f64, t9043: f64, t9046: f64, t9056: f64, t9060: f64, t9267: f64, t9276: f64, t98: f64) -> f64 {
    let t9330 = t9291 + t9301 + t9313 + t9328;
    let t9331 = t83 * t9330;
    let t9332 = t9331 * t89;
    let t9335 = t4398 - 0.10237773105191754_f64 * t3317 - 0.10237773105191754_f64 * t3319 - 0.06825182070127836_f64 * t3323 - 0.06825182070127836_f64 * t3326 - t4406 - t1052 * t9043 / 3.0_f64 - t1052 * t9046 / 6.0_f64 + t1059 * t8892 / 6.0_f64 - t9267 / 6.0_f64 - t1052 * t9056 / 6.0_f64 - t1052 * t9060 / 3.0_f64 + t1059 * t8895 / 6.0_f64 + t9276 / 6.0_f64 - t9332 * t98 / 6.0_f64;
    t9335
}
