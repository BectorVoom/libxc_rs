//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1188/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1188(t2639: f64, t955: f64, t2645: f64, t405: f64, t6152: f64, t4913: f64, t6156: f64, t103: f64, t15373: f64, t15378: f64, t15382: f64, t15387: f64, t15438: f64, t15442: f64, t15447: f64, t1619: f64, t1858: f64, t1863: f64, t2060: f64, t3404: f64, t473: f64, t9693: f64) -> f64 {
    let t15663 = t955 * t2639;
    let t15671 = t955 * t2645;
    let t15675 = t405 * t6152;
    let t15677 = t4913 * t6156;
    let t15692 = -0.0024691358024691358_f64 * t15663 - 0.008888888888888889_f64 * t2060 * t1619 * t1858 + 0.05333333333333334_f64 * t2060 * t473 * t1863 - 0.007407407407407408_f64 * t15671 + 0.2879333333333333_f64 * t15438 - 0.8638_f64 * t15442 + 0.003950617283950617_f64 * t15675 + 0.03851851851851852_f64 * t15677 + 0.013333333333333334_f64 * t103 * t1619 * t15373 + 0.035555555555555556_f64 * t103 * t3404 * t15378 - 0.002962962962962963_f64 * t103 * t3404 * t15382 - 0.006913580246913581_f64 * t103 * t9693 * t15387 + 0.14396666666666666_f64 * t15447;
    t15692
}
