//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 558/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk558<F: Float>(t10: F, t127: F, t1568: F, t3222: F, t3251: F, t3280: F, t3282: F, t3284: F, t3288: F, t3290: F, t3291: F, t3296: F, t3302: F, t3305: F, t3313: F, t3314: F, t3322: F, t3325: F, t426: F, t436: F) -> (F,) {
    let t3326 = t3280 - t3282 - t3284 - t3288 - t3290 + 9.0 / 2.0 * t426 * t10 * t3291 - 29.3808 * t127 * t3296 * t3222 - t3302 - t3305 - 1.46904 * t127 * t436 * t3251 + t3313 + 17.62848 * t127 * t3314 * t1568 - t3322 + t3325;
    (t3326,)
}
