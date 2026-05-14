//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 832/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk832<F: Float>(t1444: F, t2466: F, t1450: F, t2465: F, t493: F, t498: F, t5974: F, t496: F, t1969: F, t2002: F, t136: F, t813: F, t1968: F, t439: F, t1592: F, t2648: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6540 = t1444 * t2466 / 45.0;
    let t6541 = t1450 * t2465;
    let t6543 = t493 * t6541 / 45.0;
    let t6544 = t498 * t5974;
    let t6545 = t496 * t6544;
    let t6547 = t493 * t6545 / 45.0;
    let t6549 = 2.0 / 15.0 * t2002 * t1969;
    let t6550 = t136 * t813;
    let t6551 = t6550 * t1968;
    let t6553 = 2.0 / 15.0 * t439 * t6551;
    let t6554 = t1592 * t2648;
    (t6540, t6541, t6543, t6544, t6545, t6547, t6549, t6550, t6551, t6553, t6554)
}
