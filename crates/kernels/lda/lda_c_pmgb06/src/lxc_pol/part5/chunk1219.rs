//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1219/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1219<F: Float>(t11921: F, t19644: F, t19658: F, t19660: F, t19662: F, t19664: F, t19666: F, t19668: F, t19670: F, t19671: F, t19672: F, t19673: F) -> F {
    let t21916 = t19644 + t19658 + t19660 + t19662 + t19664 + t11921 - t19666 - t19668 + t19670 - t19671 - t19672 + t19673;
    t21916
}
