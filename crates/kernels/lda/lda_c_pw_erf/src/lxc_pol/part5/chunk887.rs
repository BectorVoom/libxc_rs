//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 887/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk887<F: Float>(t2171: F, t5397: F, t1446: F, t6696: F, t1449: F, t519: F, t6938: F, t4804: F, t6689: F, t3794: F, t518: F, t6787: F, t2146: F, t5342: F, t5339: F, t1318: F, t2531: F, t9432: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15525 = t2171 * t5397;
    let t15538 = t1446 * t6696;
    let t15542 = t519 * t1449 * t6938;
    let t15557 = t4804 * t6689;
    let t15559 = t3794 * t6689;
    let t15563 = t6787 * t518;
    let t15568 = t2146 * t5342;
    let t15570 = t2171 * t5339;
    let t15573 = t1318 * t9432 * t2531;
    (t15525, t15538, t15542, t15557, t15559, t15563, t15568, t15570, t15573)
}
