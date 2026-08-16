//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1229/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1229<F: Float>(t20129: F, t20131: F, t20133: F, t20135: F, t20138: F, t20139: F, t20140: F, t20142: F, t20143: F, t20144: F, t20146: F, t20151: F) -> F {
    let t21956 = -t20129 + t20131 + t20133 + t20135 + t20138 - t20139 - t20140 - t20142 - t20143 + t20144 + t20146 + t20151;
    t21956
}
