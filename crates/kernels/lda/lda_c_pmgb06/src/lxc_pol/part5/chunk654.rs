//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 654/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk654<F: Float>(t2064: F, t809: F, t1385: F, t439: F, t224: F, t6308: F, t6310: F, t6312: F, t6314: F, t6316: F, t6318: F, t6320: F, t6322: F, t6324: F, t6326: F, t6327: F, t6355: F, t6358: F, t6360: F, t6363: F) -> (F, F, F, F) {
    let t6364 = t809 * t2064;
    let t6365 = t1385 * t6364;
    let t6367 = 2.0 / 45.0 * t439 * t6365;
    let t6368 = -t6308 - t6310 + t6312 + t6314 + t6316 + t6318 + t6320 + t6322 + t6324 + t6326 + 2.0 / 9.0 * t6327 - t6355 * t224 / 15.0 - 2.0 / 45.0 * t6358 - t6360 - t6363 - t6367;
    (t6364, t6365, t6367, t6368)
}
