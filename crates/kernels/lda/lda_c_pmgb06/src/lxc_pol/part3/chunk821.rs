//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 821/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk821<F: Float>(t3615: F, t769: F, t4394: F, t56: F, t38: F, t370: F, t1234: F, t2229: F, t365: F, t110: F, t30: F, t342: F) -> (F, F, F, F, F, F, F) {
    let t5756 = t3615 * t769;
    let t5760 = t56 * t4394;
    let t5762 = F::cast_from(2.923025_f64) * t38 * t5760;
    let t5763 = t370 * t4394;
    let t5766 = t2229 * t1234;
    let t5770 = t365 * t2229;
    let t5772 = t30 * t110 * t342;
    (t5756, t5760, t5762, t5763, t5766, t5770, t5772)
}
