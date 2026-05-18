//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1234/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1234<F: Float>(t2550: F, t5327: F, t2171: F, t6939: F, t22189: F, t22192: F, t22194: F, t22196: F, t22200: F, t22204: F, t22207: F, t22208: F, t22210: F, t22212: F, t22214: F) -> (F, F, F) {
    let t22216 = F::new(4.0) / F::new(15.0) * t5327 * t2550;
    let t22218 = F::new(4.0) / F::new(15.0) * t2171 * t6939;
    let t22219 = -t22189 + t22192 - t22194 - t22196 - t22200 - t22204 + t22207 + t22208 + t22210 + t22212 + t22214 + t22216 + t22218;
    (t22216, t22218, t22219)
}
