//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 464/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk464<F: Float>(t123: F, t199: F, t2281: F, t125: F, t1798: F, t722: F, t868: F, t395: F, t902: F, t1155: F, t1158: F, t1161: F, t1205: F, t1206: F, t1808: F, t2164: F, t305: F, t566: F, t726: F, t81: F, t912: F) -> (F, F, F, F, F) {
    let t2283 = t123 * t2281 * t199;
    let t2285 = t125 * t1798;
    let t2293 = t123 * t722 * t868;
    let t2302 = t395 * t902;
    let t2306 = -t1155 + F::cast_from(0.053059442957798957_f64) * t1158 + F::cast_from(0.053059442957798957_f64) * t1161 + F::cast_from(0.053059442957798957_f64) * t2283 - F::cast_from(0.031835665774679375_f64) * t123 * t2285 * t199 - F::cast_from(0.031835665774679375_f64) * t123 * t912 * t566 + F::cast_from(0.053059442957798957_f64) * t2293 - F::cast_from(0.031835665774679375_f64) * t123 * t726 * t868 - F::cast_from(0.031835665774679375_f64) * t123 * t305 * t1808 + t1205 - F::cast_from(0.10665013548435875_f64) * t1206 - F::cast_from(0.10665013548435875_f64) * t2302 + F::cast_from(0.05332506774217938_f64) * t81 * t2164;
    (t2283, t2285, t2293, t2302, t2306)
}
