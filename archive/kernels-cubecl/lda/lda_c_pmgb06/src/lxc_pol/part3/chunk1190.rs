//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1190/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1190<F: Float>(t11766: F, t11770: F, t11771: F, t11772: F, t11774: F, t11775: F, t11776: F, t11778: F, t11779: F, t11780: F, t11781: F, t11782: F) -> F {
    let t14319 = -t11766 - t11770 + t11771 + t11772 + t11774 + t11775 + t11776 + t11778 + t11779 + t11780 - t11781 - t11782;
    t14319
}
