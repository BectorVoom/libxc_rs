//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1001/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1001<F: Float>(t3518: F, t3892: F, t529: F, t1245: F, t4722: F, t1251: F, t4489: F, t10030: F, t5152: F, t12064: F, t4509: F, t108: F, t267: F, t564: F, t1401: F, t1484: F) -> (F, F, F, F, F, F, F) {
    let t12380 = t3892 * t529 * t3518;
    let t12387 = t4722 * t1245;
    let t12403 = t4489 * t1251;
    let t12409 = t10030 * t5152;
    let t12411 = t12064 * t4509;
    let t12414 = t564 * t108 * t267;
    let t12428 = t1484 * t1401;
    (t12380, t12387, t12403, t12409, t12411, t12414, t12428)
}
