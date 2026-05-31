//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 886/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk886<F: Float>(t9291: F, t9301: F, t9313: F, t9328: F, t83: F, t89: F, t1052: F, t1059: F, t3317: F, t3319: F, t3323: F, t3326: F, t4398: F, t4406: F, t8892: F, t8895: F, t9043: F, t9046: F, t9056: F, t9060: F, t9267: F, t9276: F, t98: F) -> F {
    let t9330 = t9291 + t9301 + t9313 + t9328;
    let t9331 = t83 * t9330;
    let t9332 = t9331 * t89;
    let t9335 = t4398 - F::cast_from(0.10237773105191754_f64) * t3317 - F::cast_from(0.10237773105191754_f64) * t3319 - F::cast_from(0.06825182070127836_f64) * t3323 - F::cast_from(0.06825182070127836_f64) * t3326 - t4406 - t1052 * t9043 / F::cast_from(3.0_f64) - t1052 * t9046 / F::cast_from(6.0_f64) + t1059 * t8892 / F::cast_from(6.0_f64) - t9267 / F::cast_from(6.0_f64) - t1052 * t9056 / F::cast_from(6.0_f64) - t1052 * t9060 / F::cast_from(3.0_f64) + t1059 * t8895 / F::cast_from(6.0_f64) + t9276 / F::cast_from(6.0_f64) - t9332 * t98 / F::cast_from(6.0_f64);
    t9335
}
