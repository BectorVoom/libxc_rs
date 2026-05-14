//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 885/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk885<F: Float>(t1322: F, t2513: F, t5664: F, t5679: F, t5681: F, t6031: F, t6035: F, t6043: F, t6107: F, t6109: F, t6117: F, t6120: F, t6129: F, t6133: F, t6137: F, t6152: F, t6154: F, t6155: F, t6164: F, t9770: F) -> (F,) {
    let t10928 = t5679 + 0.04115066352984959 * t5681 - 4.937333717448355 * t5664 * t2513 - 4.937333717448355 * t1322 * t9770 + 6.496391258193384 * t6107 - 6.496391258193384 * t6109 - 19.489173774580152 * t6117 - t6120 - t6129 + t6133 - t6137 - t6152 - t6154 + 4.738783832122567 * t6155 - 22.07984838129906 * t6031 - 10.80049028389238 * t6035 + 10.80049028389238 * t6043 - t6164;
    (t10928,)
}
