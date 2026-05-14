//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 908/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk908<F: Float>(t132: F, t435: F, t5119: F, t3031: F, t813: F, t137: F, t3033: F, t1512: F, t2066: F, t2043: F, t1447: F, t5282: F, t2912: F, t2918: F, t2991: F, t493: F, t851: F) -> (F, F, F, F, F, F) {
    let t12191 = t132 * t435 * t5119;
    let t12192 = 2.0 / 15.0 * t12191;
    let t12193 = t813 * t3031;
    let t12197 = t132 * t137 * t12193 * t3033 / 5.0;
    let t12199 = t1512 * t2066 / 10.0;
    let t12201 = t1512 * t2043 / 10.0;
    let t12202 = t1447 * t5282;
    let t12203 = 2.0 / 27.0 * t12202;
    let t12208 = 2.0 / 9.0 * t493 * t2991 * t851 * t2918 * t2912;
    (t12192, t12197, t12199, t12201, t12203, t12208)
}
