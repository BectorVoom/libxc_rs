//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1285/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1285<F: Float>(t14978: F, t14980: F, t15742: F, t15744: F, t15748: F, t15751: F, t15755: F, t15759: F, t15763: F, t15765: F, t15767: F, t15907: F, t15911: F, t15912: F, t15913: F, t15917: F, t15918: F) -> (F,) {
    let t19115 = -t15742 + t15744 + t15748 + t15751 + t15755 + t15759 + t15763 + t15765 - t15767 + t15907 + t15911 + t15912 - t15913 + 0.13298177777777778 * t14978 + 0.19947266666666666 * t14980 - t15917 - t15918;
    (t19115,)
}
