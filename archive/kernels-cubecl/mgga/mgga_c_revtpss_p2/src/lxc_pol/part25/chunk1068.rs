//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1068/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1068<F: Float>(t12803: F, t5352: F, t3720: F, t1209: F, t3781: F, t5330: F, t3153: F, t3601: F, t12269: F, t247: F, t3618: F, t12277: F, t1264: F) -> (F, F, F, F, F, F) {
    let t12804 = t12803 * t5352;
    let t12805 = t3720 * t12804;
    let t12808 = t1209 * t3781;
    let t12809 = t12808 * t5330;
    let t12810 = t3601 * t3153;
    let t12811 = t12810 * t5352;
    let t12812 = t3720 * t12811;
    let t12816 = t247 * t3618 * t12269;
    let t12822 = t247 * t1264 * t12277;
    (t12805, t12809, t12810, t12812, t12816, t12822)
}
