//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 594/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk594<F: Float>(t107: F, t2060: F, t247: F, t2781: F, t2786: F, t93: F) -> (F,) {
    let t2789 = 7.0 / 27.0 * t93 * t2781 - 0.06068888888888889 * t2060 + 0.01829167760955153 * t247 - 0.0036147222222222223 * t107 * t2786;
    (t2789,)
}
