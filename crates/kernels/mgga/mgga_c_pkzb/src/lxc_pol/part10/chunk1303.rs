//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1303/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1303<F: Float>(t1862: F, t3532: F, t5511: F, t17444: F, t3528: F, t2754: F, t7360: F, t1861: F, t667: F, t9164: F, t1867: F, t9142: F, t9171: F, t2765: F, t1873: F, t9177: F) -> (F, F, F, F, F, F, F, F, F) {
    let t25785 = t5511 * t3532 * t1862;
    let t25788 = t17444 * t3528 * t1862;
    let t25790 = t2754 * t7360;
    let t25793 = t1861 * t9164 * t667;
    let t25795 = t9142 * t1867;
    let t25797 = t9171 * t1867;
    let t25799 = t2765 * t7360;
    let t25802 = t1873 * t9164 * t667;
    let t25804 = t9177 * t1867;
    (t25785, t25788, t25790, t25793, t25795, t25797, t25799, t25802, t25804)
}
