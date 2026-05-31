//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 771/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk771<F: Float>(t127: F, t3217: F, t3228: F, t3260: F, t3280: F, t3282: F, t3284: F, t3288: F, t3290: F, t411: F, t5502: F, t5507: F, t5511: F, t5513: F, t5517: F, t5523: F, t7093: F, t7096: F, t7100: F, t7101: F, t7102: F, t7108: F) -> F {
    let t7109 = -F::cast_from(1.95872_f64) * t5502 - t7093 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t5507 + t5511 - F::cast_from(0.97936_f64) * t5513 + t5517 + t7096 + t5523 - F::cast_from(0.97936_f64) * t3217 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t3228 - F::cast_from(0.48968_f64) * t3260 + t7100 - t7101 + t3280 - t3282 - t3284 - t3288 - t3290 + F::cast_from(5.87616_f64) * t127 * t7102 * t411 - t7108;
    t7109
}
