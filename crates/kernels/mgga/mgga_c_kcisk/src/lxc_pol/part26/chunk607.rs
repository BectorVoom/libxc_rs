//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 607/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk607<F: Float>(t1224: F, t4013: F, t5671: F, t1225: F, t5676: F, t416: F, t918: F, t5601: F, t4008: F, t4011: F, t6020: F, t1233: F, t2115: F, t1253: F, t2129: F, t2119: F, t4037: F) -> (F, F, F, F, F, F, F, F) {
    let t6023 = t1224 * t4013 * t5671;
    let t6026 = t1224 * t1225 * t5676;
    let t6028 = t918 * t416;
    let t6030 = t1224 * t6028 * t5601;
    let t6032 = t4008 + 0.5936111111111111111e-2 * t4011 + 0.5936111111111111111e-2 * t6020 - 0.11872222222222222222e-1 * t6023 + 0.35616666666666666666e-1 * t6026 - 0.35616666666666666666e-1 * t6030;
    let t6035 = t2115 * t1233;
    let t6040 = t2129 * t1253;
    let t6043 = t4037 * t2119;
    (t6023, t6026, t6028, t6030, t6032, t6035, t6040, t6043)
}
