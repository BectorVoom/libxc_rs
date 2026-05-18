//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1227/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1227<F: Float>(t12650: F, t20071: F, t20074: F, t20076: F, t20079: F, t20081: F, t20084: F, t20086: F, t20089: F, t20090: F, t20104: F, t20107: F) -> F {
    let t21951 = t20071 + t20074 + t20076 + t20079 + t20081 - t12650 + t20084 + t20086 + t20089 - t20090 + t20104 + t20107;
    t21951
}
