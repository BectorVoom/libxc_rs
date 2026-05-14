//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 721/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk721<F: Float>(t1981: F, t5322: F, t3306: F, t2065: F, t435: F, t132: F, t2015: F, t432: F, t1596: F, t802: F, t1915: F, t4861: F, t493: F, t1602: F, t838: F, t2871: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5324 = 4.0 / 45.0 * t1981 * t5322;
    let t5325 = 2.0 / 135.0 * t3306;
    let t5326 = t435 * t2065;
    let t5328 = 2.0 / 45.0 * t132 * t5326;
    let t5330 = 2.0 / 45.0 * t432 * t2015;
    let t5332 = t802 * t1596 / 15.0;
    let t5333 = t1915 * t4861;
    let t5335 = 2.0 / 15.0 * t493 * t5333;
    let t5336 = t838 * t1602;
    let t5337 = t2871 * t5336;
    (t5324, t5325, t5326, t5328, t5330, t5332, t5333, t5335, t5336, t5337)
}
