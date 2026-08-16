//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1438/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1438<F: Float>(t17873: F, t17876: F, t17878: F, t17879: F, t17880: F, t17884: F, t17885: F, t17887: F, t17889: F, t17891: F, t17895: F, t17898: F, t17902: F, t17904: F, t17906: F) -> F {
    let t18389 = t17873 - t17876 - t17878 - t17879 + t17880 - t17884 - t17885 - t17887 + t17889 + t17891 + t17895 + t17898 + t17902 - t17904 - t17906;
    t18389
}
