//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1004/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1004<F: Float>(t11691: F, t2594: F, t11694: F, t7296: F, t5213: F, t7444: F, t5219: F, t11701: F, t1957: F, t5218: F, t5339: F, t1931: F, t7330: F, t1954: F, t7410: F, t2572: F, t5274: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t17779 = t11691 * t2594;
    let t17781 = 4.0 * t11694 * t7296;
    let t17783 = 2.0 * t5213 * t7444;
    let t17784 = t2594 * t5219;
    let t17786 = 6.0 * t11701 * t17784;
    let t17787 = t7444 * t1957;
    let t17789 = 4.0 * t5218 * t17787;
    let t17790 = t2594 * t5339;
    let t17792 = 2.0 * t5218 * t17790;
    let t17793 = t1931 * t7330;
    let t17795 = t7410 * t1954;
    let t17797 = t5274 * t2572;
    (t17779, t17781, t17783, t17784, t17786, t17787, t17789, t17790, t17792, t17793, t17795, t17797)
}
