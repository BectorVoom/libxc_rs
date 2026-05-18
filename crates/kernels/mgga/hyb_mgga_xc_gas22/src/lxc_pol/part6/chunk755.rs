//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 755/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk755<F: Float>(t154: F, t3997: F, t4014: F, t712: F, t157: F, t716: F, t160: F, t720: F, t163: F, t724: F, t166: F, t728: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4017 = t154 * t3997;
    let t4019 = t712 * t4014;
    let t4021 = t157 * t3997;
    let t4023 = t716 * t4014;
    let t4025 = t160 * t3997;
    let t4027 = t720 * t4014;
    let t4029 = t163 * t3997;
    let t4031 = t724 * t4014;
    let t4033 = t166 * t3997;
    let t4035 = t728 * t4014;
    (t4017, t4019, t4021, t4023, t4025, t4027, t4029, t4031, t4033, t4035)
}
