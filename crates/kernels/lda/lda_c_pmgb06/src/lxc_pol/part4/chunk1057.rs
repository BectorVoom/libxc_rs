//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1057/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1057<F: Float>(t436: F, t6705: F, t1517: F, t2592: F, t11875: F, t11881: F, t11884: F, t15771: F, t15773: F, t15775: F, t15777: F, t15780: F, t15784: F, t15786: F, t15789: F, t15792: F) -> (F, F, F, F, F, F) {
    let t15793 = t6705 * t436;
    let t15794 = 2.0 / 45.0 * t15793;
    let t15795 = t2592 * t1517;
    let t15796 = 2.0 / 45.0 * t15795;
    let t15797 = 8.0 / 135.0 * t11875;
    let t15798 = 16.0 / 405.0 * t11881;
    let t15799 = 8.0 / 405.0 * t11884;
    let t15800 = t15771 - t15773 + t15775 + t15777 + t15780 + t15784 + t15786 + t15789 + t15792 + t15794 + t15796 - t15797 + t15798 + t15799;
    (t15794, t15796, t15797, t15798, t15799, t15800)
}
