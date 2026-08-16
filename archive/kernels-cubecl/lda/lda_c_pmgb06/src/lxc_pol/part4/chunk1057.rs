//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1057/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1057<F: Float>(t1271: F, t2233: F, t955: F, t348: F, t350: F, t5760: F, t1238: F, t2210: F, t110: F, t5809: F, t360: F, t1830: F, t2226: F) -> (F, F, F, F, F, F) {
    let t11373 = t1271 * t2233 * t955;
    let t11376 = t348 * t5760 * t350;
    let t11379 = t1238 * t2210 * t955;
    let t11381 = t110 * t5809;
    let t11382 = t360 * t11381;
    let t11388 = t2226 * t1830;
    (t11373, t11376, t11379, t11381, t11382, t11388)
}
