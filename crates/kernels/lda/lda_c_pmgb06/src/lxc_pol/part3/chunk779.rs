//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 779/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk779<F: Float>(t1586: F, t1993: F, t1992: F, t493: F, t1450: F, t1982: F, t1981: F, t3306: F, t2065: F, t435: F, t132: F, t2015: F, t432: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5318 = t1993 * t1586;
    let t5319 = t1992 * t5318;
    let t5321 = t493 * t5319 / F::new(15.0);
    let t5322 = t1450 * t1982;
    let t5324 = F::new(4.0) / F::new(45.0) * t1981 * t5322;
    let t5325 = F::new(2.0) / F::new(135.0) * t3306;
    let t5326 = t435 * t2065;
    let t5328 = F::new(2.0) / F::new(45.0) * t132 * t5326;
    let t5330 = F::new(2.0) / F::new(45.0) * t432 * t2015;
    (t5318, t5319, t5321, t5322, t5324, t5325, t5326, t5328, t5330)
}
