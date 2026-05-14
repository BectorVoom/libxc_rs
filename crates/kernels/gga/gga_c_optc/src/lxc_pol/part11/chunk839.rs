//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 839/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk839<F: Float>(t16858: F, t779: F, t1342: F, t4818: F, t7672: F, t7669: F, t10416: F, t4898: F, t2418: F, t7681: F, t16708: F, t818: F, t799: F, t2416: F, t16784: F, t837: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t16860 = 1.0 * t779 * t16858;
    let t16861 = t4818 * t1342;
    let t16862 = t16861 * t7672;
    let t16864 = 0.51725014705706168417e3 * t7669 * t16862;
    let t16866 = 6.0 * t10416 * t4898;
    let t16867 = t16861 * t2418;
    let t16869 = 0.96490945932906628932e2 * t7681 * t16867;
    let t16872 = t16708 * t818;
    let t16875 = t16861 * t799;
    let t16877 = 6.0 * t2416 * t16875;
    let t16880 = t16784 * t837;
    (t16860, t16862, t16864, t16866, t16867, t16869, t16872, t16875, t16877, t16880)
}
