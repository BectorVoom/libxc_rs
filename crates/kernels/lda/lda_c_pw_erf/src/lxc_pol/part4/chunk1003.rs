//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1003/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1003<F: Float>(t3384: F, t795: F, t184: F, t202: F, t4701: F, t1621: F, t1931: F, t4233: F, t838: F, t4714: F, t611: F, t348: F, t494: F, t1318: F, t3899: F, t5355: F) -> (F, F, F, F, F, F, F) {
    let t12498 = t795 * t3384;
    let t12501 = t202 * t4701 * t184;
    let t12507 = t1931 * t1621;
    let t12509 = t838 * t4233;
    let t12514 = t4714 * t611;
    let t12516 = t348 * t494;
    let t12527 = t1318 * t3899 * t5355;
    (t12498, t12501, t12507, t12509, t12514, t12516, t12527)
}
