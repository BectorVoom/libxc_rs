//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1127/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1127<F: Float>(t13117: F, t13119: F, t5326: F, t802: F, t432: F, t6621: F, t1554: F, t161: F, t2653: F, t1512: F, t2584: F, t1848: F, t2018: F, t13139: F, t337: F, t529: F, t6560: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16873 = 4.0 / 45.0 * t13117;
    let t16874 = 8.0 / 45.0 * t13119;
    let t16875 = t802 * t5326;
    let t16876 = 4.0 / 45.0 * t16875;
    let t16877 = t432 * t6621;
    let t16878 = 2.0 / 45.0 * t16877;
    let t16880 = t161 * t1554 * t2653;
    let t16881 = 2.0 / 135.0 * t16880;
    let t16883 = t1512 * t2584 / 30.0;
    let t16884 = t1848 * t2018;
    let t16885 = 4.0 / 45.0 * t16884;
    let t16886 = 8.0 / 135.0 * t13139;
    let t16888 = t6560 * t529 * t337;
    (t16873, t16874, t16876, t16878, t16881, t16883, t16885, t16886, t16888)
}
