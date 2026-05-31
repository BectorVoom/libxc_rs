//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1466/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1466<F: Float>(t11475: F, t2247: F, t7073: F, t5858: F, t7077: F, t11470: F, t1227: F, t1234: F, t18704: F, t18706: F, t18707: F, t18716: F, t18729: F, t18732: F, t18735: F, t18748: F, t18750: F, t2248: F, t2448: F, t2695: F, t342: F, t4394: F, t5874: F, t5980: F, t769: F, t8339: F) -> F {
    let t18848 = t2247 * t11475 * t7073;
    let t18851 = t2247 * t5858 * t7077;
    let t18869 = -t8339 + t18704 + t18706 - t18707 + F::cast_from(10.34553_f64) * t2247 * t2248 * t5980 * t342 + F::cast_from(13.79404_f64) * t18848 - F::cast_from(6.89702_f64) * t18851 - F::cast_from(20.69106_f64) * t2247 * t5874 * t2695 * t1227 + F::cast_from(10.34553_f64) * t2247 * t2248 * t769 * t4394 + F::cast_from(103.4553_f64) * t2247 * t11470 * t2695 * t1234 - F::cast_from(20.69106_f64) * t2247 * t5874 * t2448 * t1234 - t18716 - t18729 + t18732 + t18735 + t18748 - t18750;
    t18869
}
