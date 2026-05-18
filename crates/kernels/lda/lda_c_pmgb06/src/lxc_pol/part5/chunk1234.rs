//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1234/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1234<F: Float>(t20283: F, t20284: F, t20285: F, t20288: F, t20293: F, t20296: F, t20299: F, t20302: F, t20305: F, t20308: F, t20311: F, t20314: F) -> F {
    let t21982 = -t20283 - t20284 + t20285 + t20288 + t20293 - t20296 - t20299 + t20302 + t20305 - t20308 - t20311 + t20314;
    t21982
}
