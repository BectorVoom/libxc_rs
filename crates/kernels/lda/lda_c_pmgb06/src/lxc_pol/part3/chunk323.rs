//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 323/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk323<F: Float>(t113: F, t1139: F, t301: F, t413: F, t718: F, t100: F, t246: F) -> (F, F, F) {
    let t1141 = t1139 * t113 * t301;
    let t1145 = 0.0005811348303577384 * t718 * t413 * t301;
    let t1147 = 1.0 / t100 / t246;
    (t1141, t1145, t1147)
}
