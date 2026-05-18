//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1054/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1054<F: Float>(t342: F, t569: F, t99: F, t1271: F, t2229: F, t2221: F, t348: F, t5772: F, t1238: F, t776: F, t110: F, t360: F, t5775: F) -> (F, F, F, F) {
    let t11303 = t99 * t569 * t342;
    let t11304 = t1271 * t2229 * t11303;
    let t11307 = t348 * t2221 * t5772;
    let t11310 = t1238 * t776 * t11303;
    let t11313 = t360 * t110 * t5775;
    (t11304, t11307, t11310, t11313)
}
