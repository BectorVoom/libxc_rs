//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 934/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk934<F: Float>(t2912: F, t764: F, t9509: F, t12592: F, t493: F, t1: F, t1080: F, t2911: F, t1981: F, t5470: F, t1423: F, t5233: F, t1825: F, t2938: F, t1915: F, t1972: F, t3300: F) -> (F, F, F, F, F, F, F, F) {
    let t12594 = t9509 * t764 * t2912;
    let t12597 = 88.0 / 243.0 * t493 * t12592 * t12594;
    let t12599 = t2911 * t1 * t1080;
    let t12602 = 16.0 / 27.0 * t1981 * t5470 * t12599;
    let t12603 = t1423 * t5233;
    let t12604 = 4.0 / 45.0 * t12603;
    let t12605 = t1825 * t2938;
    let t12608 = 2.0 / 45.0 * t493 * t1915 * t12605;
    let t12610 = t1972 * t3300 / 9.0;
    (t12594, t12597, t12599, t12602, t12604, t12605, t12608, t12610)
}
