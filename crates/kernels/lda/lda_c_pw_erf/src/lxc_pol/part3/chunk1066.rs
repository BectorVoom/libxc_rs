//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1066/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1066<F: Float>(t12450: F, t3965: F, t5141: F, t12025: F, t12389: F, t12476: F, t348: F, t12475: F, t4576: F, t565: F, t3384: F, t795: F) -> (F, F, F, F, F, F) {
    let t12488 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t3965 * t5141 * t12450;
    let t12491 = F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t3965 * t12025 * t12389;
    let t12492 = t12476 * t348;
    let t12495 = F::cast_from(64.0_f64) / F::cast_from(15.0_f64) * t12475 * t5141 * t12492;
    let t12497 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t565 * t4576;
    let t12498 = t795 * t3384;
    (t12488, t12491, t12492, t12495, t12497, t12498)
}
