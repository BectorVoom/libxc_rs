//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1204/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1204<F: Float>(t10315: F, t10361: F, t1529: F, t2402: F, t1522: F, t184: F, t221: F, t2423: F, t13419: F, t13422: F, t13426: F, t13428: F, t17776: F, t17778: F, t17780: F, t17782: F, t17784: F, t17786: F, t17789: F, t17790: F, t17791: F) -> (F, F, F, F, F, F, F, F, F) {
    let t17792 = 8.0 / 243.0 * t10315;
    let t17793 = 16.0 / 405.0 * t10361;
    let t17794 = t2402 * t1529;
    let t17795 = 8.0 / 135.0 * t17794;
    let t17799 = 4.0 / 15.0 * t1522 * t2423 * t184 * t221;
    let t17800 = 64.0 / 405.0 * t13419;
    let t17801 = 64.0 / 135.0 * t13422;
    let t17802 = 32.0 / 135.0 * t13426;
    let t17803 = 64.0 / 135.0 * t13428;
    let t17804 = t17776 - t17778 - t17780 - t17782 + t17784 + t17786 + t17789 + t17790 - t17791 - t17792 + t17793 - t17795 + t17799 - t17800 - t17801 - t17802 - t17803;
    (t17792, t17793, t17795, t17799, t17800, t17801, t17802, t17803, t17804)
}
