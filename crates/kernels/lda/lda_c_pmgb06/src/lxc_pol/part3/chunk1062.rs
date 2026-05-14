//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1062/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1062<F: Float>(t13856: F, t13861: F, t13863: F, t13865: F, t13867: F, t13869: F, t13872: F, t13875: F, t13878: F, t13882: F, t13884: F, t13886: F, t13888: F, t13892: F, t13894: F, t13896: F, t13899: F, t13904: F, t13906: F, t13908: F, t13910: F, t13912: F, t13914: F) -> (F, F) {
    let t14458 = -t13856 + t13861 - t13863 - t13865 - t13867 + t13869 - t13872 - t13875 + t13878 - t13882 - t13884;
    let t14459 = -t13886 - t13888 - t13892 - t13894 - t13896 + t13899 + t13904 - t13906 - t13908 + t13910 - t13912 - t13914;
    (t14458, t14459)
}
