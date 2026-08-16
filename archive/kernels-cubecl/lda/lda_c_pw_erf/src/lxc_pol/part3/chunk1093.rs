//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1093/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1093<F: Float>(t12781: F, t1325: F, t5291: F, t12756: F, t12757: F, t12758: F, t12759: F, t12760: F, t12761: F, t12762: F, t12763: F, t12764: F, t12770: F, t12775: F, t12780: F) -> (F, F) {
    let t12783 = t1325 * t12781 * t5291;
    let t12784 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t12783;
    let t12785 = t12756 - t12757 - t12758 - t12759 + t12760 - t12761 - t12762 - t12763 + t12764 - t12770 - t12775 + t12780 + t12784;
    (t12784, t12785)
}
