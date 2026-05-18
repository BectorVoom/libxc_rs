//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 783/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk783<F: Float>(t2000: F, t954: F, t1319: F, t1318: F, t1351: F, t811: F, t951: F, t2017: F, t1972: F, t3859: F, t519: F, t197: F, t3883: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5225 = t2000 * t954;
    let t5226 = t1319 * t5225;
    let t5228 = F::new(8.0) / F::new(45.0) * t1318 * t5226;
    let t5229 = t811 * t1351;
    let t5230 = t5229 * t951;
    let t5231 = t2017 * t5230;
    let t5233 = F::new(8.0) / F::new(27.0) * t1318 * t5231;
    let t5234 = t3859 * t1972;
    let t5236 = F::new(32.0) / F::new(135.0) * t519 * t5234;
    let t5237 = t3883 * t197;
    (t5225, t5226, t5228, t5229, t5230, t5231, t5233, t5234, t5236, t5237)
}
