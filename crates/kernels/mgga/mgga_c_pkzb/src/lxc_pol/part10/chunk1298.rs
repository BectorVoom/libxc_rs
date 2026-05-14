//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1298/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1298<F: Float>(t1976: F, t9203: F, t2874: F, t730: F, t1987: F, t9352: F, t9533: F, t2870: F, t7560: F, t17351: F, t17354: F, t17728: F, t20705: F, t20716: F, t20719: F, t228: F, t25633: F, t25636: F, t25639: F) -> (F, F, F, F, F) {
    let t25671 = t1976 * t9203;
    let t25674 = 0.34631718211362927518e2 * t730 * t25671 * t2874;
    let t25676 = 0.34631718211362927518e2 * t1987 * t9352;
    let t25678 = 0.20508037716432813316e4 * t1987 * t9533;
    let t25680 = 0.23392894490538584828e1 * t7560 * t2870;
    let t25691 = 0.621814e-1 * (t17728 - 0.11080740740740740741e0 * t17351 + 0.23744444444444444444e-1 * t17354 - 0.11080740740740740741e0 * t20705 + 0.94977777777777777776e-1 * t20716 - 0.35616666666666666666e-1 * t20719 + 0.23744444444444444444e-1 * t25633 - 0.35616666666666666666e-1 * t25636 + 0.53425e-1 * t25639) * t228;
    (t25674, t25676, t25678, t25680, t25691)
}
