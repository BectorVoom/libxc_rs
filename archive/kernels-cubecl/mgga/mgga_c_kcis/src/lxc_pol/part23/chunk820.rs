//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 820/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk820<F: Float>(t1464: F, t15900: F, t3805: F, t5632: F, t1395: F, t1394: F, t1017: F, t541: F, t86: F, t2011: F, t4134: F, t4129: F) -> (F, F, F, F, F) {
    let t15901 = t1464 * t15900;
    let t15903 = t5632 * t3805;
    let t15904 = t1395 * t15903;
    let t15905 = t1394 * t15904;
    let t15909 = t86 * t1017 * t541;
    let t15910 = t4134 * t2011;
    let t15911 = t15910 * t4129;
    (t15901, t15905, t15909, t15910, t15911)
}
