//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1104/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1104<F: Float>(t12749: F, t12750: F, t12751: F, t12752: F, t12753: F, t12754: F, t12756: F, t12757: F, t12758: F, t12759: F, t12760: F, t12761: F, t12762: F, t12763: F, t12764: F, t12770: F, t12775: F, t12780: F, t12784: F, t12788: F, t12793: F, t12796: F, t12799: F, t12801: F, t12803: F, t12807: F, t12810: F) -> (F, F) {
    let t15046 = t12749 - t12750 - t12751 - t12752 - t12753 + t12754 + t12756 - t12757 - t12758 - t12759 + t12760 - t12761 - t12762;
    let t15047 = -t12763 + t12764 - t12770 - t12775 + t12780 + t12784 + t12788 + t12793 + t12796 + t12799 - t12801 + t12803 - t12807 - t12810;
    (t15046, t15047)
}
