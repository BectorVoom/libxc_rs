//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1007/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1007<F: Float>(t107: F, t2786: F, t701: F, t290: F, t8170: F, t1436: F, t1592: F, t1533: F, t947: F, t139: F, t1767: F, t134: F, t138: F) -> (F, F, F, F, F, F) {
    let t9066 = t107 * t2786 * t701;
    let t9070 = F::new(19.1926369973667) * t107 * t8170 * t290;
    let t9084 = t1436 * t1592;
    let t9147 = t947 * t1533;
    let t9175 = t1767 * t139;
    let t9177 = t138 * t9175 * t134;
    (t9066, t9070, t9084, t9147, t9175, t9177)
}
