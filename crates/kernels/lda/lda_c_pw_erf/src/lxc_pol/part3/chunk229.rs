//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 229/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk229<F: Float>(t242: F, t458: F, t3: F, t483: F, t156: F, t161: F, t478: F) -> (F, F, F, F) {
    let t624 = F::cast_from(0.0837628205355044_f64) * t458 * t242;
    let t628 = t483 * t3;
    let t629 = t156 * t161;
    let t632 = t478 / F::cast_from(2.0_f64) + F::cast_from(0.03135_f64) * t628 * t629;
    (t624, t628, t629, t632)
}
