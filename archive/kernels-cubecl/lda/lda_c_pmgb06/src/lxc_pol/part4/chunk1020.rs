//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1020/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1020<F: Float>(t1382: F, t3223: F, t3194: F, t517: F, t2060: F, t526: F, t1580: F, t955: F, t1583: F, t1577: F, t1414: F, t147: F, t163: F) -> (F, F, F, F, F, F, F) {
    let t9921 = t3223 * t1382;
    let t9925 = t3194 * t517;
    let t9938 = t2060 * t526;
    let t9954 = t955 * t1580;
    let t9956 = t955 * t1583;
    let t9958 = t955 * t1577;
    let t9967 = t147 / t163 / t1414;
    (t9921, t9925, t9938, t9954, t9956, t9958, t9967)
}
