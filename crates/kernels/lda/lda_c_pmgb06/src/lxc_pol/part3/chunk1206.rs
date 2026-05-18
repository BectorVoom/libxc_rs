//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1206/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1206<F: Float>(t12859: F, t12863: F, t12867: F, t12869: F, t12871: F, t12879: F, t12881: F, t12883: F, t12885: F, t12887: F, t12889: F, t12892: F, t12894: F, t12895: F, t12899: F, t12900: F, t12902: F, t12907: F, t12909: F, t12911: F, t12914: F, t12916: F, t12918: F) -> (F, F) {
    let t14395 = -t12859 - t12863 - t12867 + t12869 + t12871 - t12879 - t12881 - t12883 - t12885 + t12887 - t12889;
    let t14396 = -t12892 - t12894 + t12895 + t12899 + t12900 + t12902 + t12907 + t12909 + t12911 + t12914 + t12916 + t12918;
    (t14395, t14396)
}
