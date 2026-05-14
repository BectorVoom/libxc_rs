//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 895/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk895<F: Float>(t153: F, t1864: F, t3216: F, t439: F, t1444: F, t5333: F, t4861: F, t493: F, t5447: F, t2912: F, t4884: F, t1919: F, t11964: F, t11970: F, t11972: F, t11974: F, t11977: F, t11981: F, t9426: F, t9429: F) -> (F, F, F, F, F, F) {
    let t11985 = 2.0 / 15.0 * t439 * t3216 * t153 * t1864;
    let t11987 = 2.0 / 5.0 * t1444 * t5333;
    let t11990 = 2.0 / 5.0 * t493 * t5447 * t4861;
    let t11991 = t4884 * t2912;
    let t11994 = 4.0 / 3.0 * t493 * t1919 * t11991;
    let t11995 = t11964 + 8.0 / 27.0 * t9426 + t9429 - t11970 - t11972 + t11974 + t11977 - t11981 - t11985 + t11987 + t11990 + t11994;
    (t11985, t11987, t11990, t11991, t11994, t11995)
}
