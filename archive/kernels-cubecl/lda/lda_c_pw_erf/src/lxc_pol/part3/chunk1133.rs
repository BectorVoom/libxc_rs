//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1133/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1133<F: Float>(t2967: F, t3604: F, t3832: F, t571: F, t833: F, t1446: F, t5426: F, t13239: F, t13242: F, t13245: F, t13248: F, t13251: F, t13253: F, t13256: F, t13259: F, t13262: F, t13264: F, t13269: F) -> (F, F, F) {
    let t13274 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t571 * t3832 * t833 * t3604 * t2967;
    let t13276 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t1446 * t5426;
    let t13277 = -t13239 - t13242 + t13245 + t13248 + t13251 - t13253 - t13256 - t13259 - t13262 + t13264 + t13269 + t13274 + t13276;
    (t13274, t13276, t13277)
}
