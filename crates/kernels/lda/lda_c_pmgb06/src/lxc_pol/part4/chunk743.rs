//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 743/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk743<F: Float>(t499: F, t5312: F, t493: F, t1444: F, t1989: F, t1586: F, t1993: F, t1992: F, t1450: F, t1982: F, t1981: F, t3306: F, t2065: F, t435: F, t132: F, t2015: F, t432: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5313 = t5312 * t499;
    let t5315 = 2.0 / 45.0 * t493 * t5313;
    let t5317 = 2.0 / 45.0 * t1444 * t1989;
    let t5318 = t1993 * t1586;
    let t5319 = t1992 * t5318;
    let t5321 = t493 * t5319 / 15.0;
    let t5322 = t1450 * t1982;
    let t5324 = 4.0 / 45.0 * t1981 * t5322;
    let t5325 = 2.0 / 135.0 * t3306;
    let t5326 = t435 * t2065;
    let t5328 = 2.0 / 45.0 * t132 * t5326;
    let t5330 = 2.0 / 45.0 * t432 * t2015;
    (t5313, t5315, t5317, t5318, t5319, t5321, t5322, t5324, t5325, t5326, t5328, t5330)
}
