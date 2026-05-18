//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 528/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk528<F: Float>(t1155: F, t1158: F, t1161: F, t1205: F, t1206: F, t123: F, t1808: F, t199: F, t2164: F, t2283: F, t2285: F, t2293: F, t2302: F, t305: F, t566: F, t726: F, t81: F, t868: F, t912: F) -> F {
    let t2306 = -t1155 + F::new(0.053059442957798957) * t1158 + F::new(0.053059442957798957) * t1161 + F::new(0.053059442957798957) * t2283 - F::new(0.031835665774679375) * t123 * t2285 * t199 - F::new(0.031835665774679375) * t123 * t912 * t566 + F::new(0.053059442957798957) * t2293 - F::new(0.031835665774679375) * t123 * t726 * t868 - F::new(0.031835665774679375) * t123 * t305 * t1808 + t1205 - F::new(0.10665013548435875) * t1206 - F::new(0.10665013548435875) * t2302 + F::new(0.05332506774217938) * t81 * t2164;
    t2306
}
