//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 764/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk764<F: Float>(t6214: F, t6261: F, t6311: F, t6340: F, t6412: F, t6472: F, t6595: F, t6674: F, t6688: F, t6746: F, t6784: F, t6854: F, t6893: F, t6936: F, t6996: F, t7021: F) -> F {
    let t7025 = t6214 + t6261 + t6311 + t6340 + t6412 + t6472 + t6595 + t6674 + t6688 + t6746 + t6784 + t6854 + t6893 + t6936 + t6996 + t7021;
    t7025
}
