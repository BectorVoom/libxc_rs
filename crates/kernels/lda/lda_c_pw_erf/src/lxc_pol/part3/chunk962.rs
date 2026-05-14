//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 962/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk962<F: Float>(t1318: F, t1381: F, t5269: F, t593: F, t811: F, t1390: F, t3787: F, t1325: F, t5291: F, t12756: F, t12757: F, t12758: F, t12759: F, t12760: F, t12761: F, t12762: F, t12763: F, t12764: F, t12770: F, t12775: F) -> (F, F, F) {
    let t12780 = 8.0 / 5.0 * t1318 * t5269 * t811 * t593 * t1381;
    let t12781 = t3787 * t1390;
    let t12783 = t1325 * t12781 * t5291;
    let t12784 = 32.0 / 15.0 * t12783;
    let t12785 = t12756 - t12757 - t12758 - t12759 + t12760 - t12761 - t12762 - t12763 + t12764 - t12770 - t12775 + t12780 + t12784;
    (t12780, t12784, t12785)
}
