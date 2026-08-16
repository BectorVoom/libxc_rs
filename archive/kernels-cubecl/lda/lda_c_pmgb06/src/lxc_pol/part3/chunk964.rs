//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 964/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk964<F: Float>(t350: F, t365: F, t5763: F, t1271: F, t2233: F, t955: F, t348: F, t5760: F, t1238: F, t2210: F, t110: F, t5809: F) -> (F, F, F, F, F) {
    let t11370 = t365 * t5763 * t350;
    let t11373 = t1271 * t2233 * t955;
    let t11374 = F::cast_from(1.46904_f64) * t11373;
    let t11376 = t348 * t5760 * t350;
    let t11377 = F::cast_from(1.4615125_f64) * t11376;
    let t11379 = t1238 * t2210 * t955;
    let t11380 = F::cast_from(0.9743416666666667_f64) * t11379;
    let t11381 = t110 * t5809;
    (t11370, t11374, t11377, t11380, t11381)
}
