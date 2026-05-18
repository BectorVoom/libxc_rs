//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1422/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1422<F: Float>(t10079: F, t10082: F, t10085: F, t17257: F, t17258: F, t17259: F, t17261: F, t17262: F, t17263: F, t17264: F, t17265: F, t17266: F, t17268: F, t17272: F, t17275: F) -> F {
    let t18299 = t17257 + t17258 + t17259 - t17261 - t17262 - t17263 - t17264 - F::new(16.0) / F::new(405.0) * t10079 + t10082 + F::new(2.0) / F::new(135.0) * t10085 - t17265 - t17266 + t17268 + t17272 + t17275;
    t18299
}
