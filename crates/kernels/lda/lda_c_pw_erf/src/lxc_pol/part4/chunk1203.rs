//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1203/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1203<F: Float>(t1405: F, t2402: F, t12881: F, t2558: F, t12641: F, t4804: F, t6917: F, t15926: F, t4895: F, t1476: F, t7007: F, t1318: F, t3899: F, t6964: F, t13401: F, t10311: F) -> (F, F, F, F, F, F, F, F, F) {
    let t17776 = 8.0 / 15.0 * t2402 * t1405;
    let t17778 = 8.0 / 15.0 * t12881 * t2558;
    let t17780 = 16.0 / 15.0 * t12641 * t2558;
    let t17782 = 16.0 / 15.0 * t4804 * t6917;
    let t17784 = 16.0 / 15.0 * t15926 * t4895;
    let t17785 = t7007 * t1476;
    let t17786 = 32.0 / 135.0 * t17785;
    let t17788 = t1318 * t3899 * t6964;
    let t17789 = 32.0 / 45.0 * t17788;
    let t17790 = 8.0 / 45.0 * t13401;
    let t17791 = 8.0 / 405.0 * t10311;
    (t17776, t17778, t17780, t17782, t17784, t17786, t17789, t17790, t17791)
}
