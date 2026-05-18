//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 481/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk481<F: Float>(t1017: F, t1021: F, t1028: F, t1038: F, t1046: F, t1098: F, t1103: F, t1107: F, t1110: F, t1114: F, t1118: F, t1124: F) -> F {
    let t2406 = -t1098 + t1103 + t1107 + t1110 - t1017 + t1114 - t1021 - t1118 - t1028 + t1038 + t1046 + t1124;
    t2406
}
