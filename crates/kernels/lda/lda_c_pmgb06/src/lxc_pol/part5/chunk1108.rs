//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1108/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1108<F: Float>(t20318: F, t2489: F, t5194: F, t20293: F, t20296: F, t20299: F, t20302: F, t20305: F, t20308: F, t20311: F, t20314: F, t20317: F) -> (F, F, F) {
    let t20319 = F::new(4.0) / F::new(45.0) * t20318;
    let t20320 = t5194 * t2489;
    let t20321 = F::new(4.0) / F::new(45.0) * t20320;
    let t20322 = t20293 - t20296 - t20299 + t20302 + t20305 - t20308 - t20311 + t20314 - t20317 - t20319 - t20321;
    (t20319, t20321, t20322)
}
