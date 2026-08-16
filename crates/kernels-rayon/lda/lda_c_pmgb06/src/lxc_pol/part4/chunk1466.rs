//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1466/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1466(t11475: f64, t2247: f64, t7073: f64, t5858: f64, t7077: f64, t11470: f64, t1227: f64, t1234: f64, t18704: f64, t18706: f64, t18707: f64, t18716: f64, t18729: f64, t18732: f64, t18735: f64, t18748: f64, t18750: f64, t2248: f64, t2448: f64, t2695: f64, t342: f64, t4394: f64, t5874: f64, t5980: f64, t769: f64, t8339: f64) -> f64 {
    let t18848 = t2247 * t11475 * t7073;
    let t18851 = t2247 * t5858 * t7077;
    let t18869 = -t8339 + t18704 + t18706 - t18707 + 10.34553_f64 * t2247 * t2248 * t5980 * t342 + 13.79404_f64 * t18848 - 6.89702_f64 * t18851 - 20.69106_f64 * t2247 * t5874 * t2695 * t1227 + 10.34553_f64 * t2247 * t2248 * t769 * t4394 + 103.4553_f64 * t2247 * t11470 * t2695 * t1234 - 20.69106_f64 * t2247 * t5874 * t2448 * t1234 - t18716 - t18729 + t18732 + t18735 + t18748 - t18750;
    t18869
}
