//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1204/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1204(t15872: f64, t5077: f64, t5084: f64, t10679: f64, t10681: f64, t10684: f64, t15836: f64, t15839: f64, t15841: f64, t15843: f64, t15849: f64, t15851: f64, t15857: f64, t15860: f64, t15864: f64, t15867: f64, t15870: f64) -> (f64, f64) {
    let t15875 = 4.0_f64 / 15.0_f64 * t5077 * t5084 * t15872;
    let t15876 = t15836 + t15839 + t15841 + t15843 + t15849 + t15851 + 0.07214027574909895_f64 * t10679 - 0.022363485482220676_f64 * t10681 - t10684 + t15857 - t15860 + t15864 - t15867 - t15870 - t15875;
    (t15875, t15876)
}
