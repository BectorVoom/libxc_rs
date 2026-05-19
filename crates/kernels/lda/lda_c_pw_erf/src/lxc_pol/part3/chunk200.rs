//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 200/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk200<F: Float>(t501: F, t171: F, t191: F, t187: F, t190: F, t177: F) -> (F, F, F, F, F) {
    let t531 = F::cast_from(0.035991666666666665_f64) * t501;
    let t533 = t171 * t191;
    let t536 = F::cast_from(0.006666666666666667_f64) * t190 * t533 * t187;
    let t537 = F::new(1.0) / t177;
    let t538 = t191 * t537;
    (t531, t533, t536, t537, t538)
}
