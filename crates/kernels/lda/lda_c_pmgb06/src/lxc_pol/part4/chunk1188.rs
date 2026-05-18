//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1188/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1188<F: Float>(t2639: F, t955: F, t2645: F, t405: F, t6152: F, t4913: F, t6156: F, t103: F, t15373: F, t15378: F, t15382: F, t15387: F, t15438: F, t15442: F, t15447: F, t1619: F, t1858: F, t1863: F, t2060: F, t3404: F, t473: F, t9693: F) -> F {
    let t15663 = t955 * t2639;
    let t15671 = t955 * t2645;
    let t15675 = t405 * t6152;
    let t15677 = t4913 * t6156;
    let t15692 = -F::new(0.0024691358024691358) * t15663 - F::new(0.008888888888888889) * t2060 * t1619 * t1858 + F::new(0.05333333333333334) * t2060 * t473 * t1863 - F::new(0.007407407407407408) * t15671 + F::new(0.2879333333333333) * t15438 - F::new(0.8638) * t15442 + F::new(0.003950617283950617) * t15675 + F::new(0.03851851851851852) * t15677 + F::new(0.013333333333333334) * t103 * t1619 * t15373 + F::new(0.035555555555555556) * t103 * t3404 * t15378 - F::new(0.002962962962962963) * t103 * t3404 * t15382 - F::new(0.006913580246913581) * t103 * t9693 * t15387 + F::new(0.14396666666666666) * t15447;
    t15692
}
