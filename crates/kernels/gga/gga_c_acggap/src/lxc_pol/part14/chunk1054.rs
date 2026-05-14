//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1054/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1054<F: Float>(t2046: F, t336: F, t5506: F, t579: F, t31751: F, t31752: F, t36126: F, t36127: F, t36132: F, t36134: F, t36137: F, t36152: F, t36157: F, t37874: F, t37877: F, t37879: F, t40425: F, t40427: F, t40431: F, t40436: F, t40442: F) -> (F,) {
    let t40446 = t2046 * t336 * t579 * t5506;
    let t40448 = -t40425 / 192.0 + t36126 + 0.28303283060643736861e-1 * t40427 + 0.15724046144802076034e-2 * t40431 + 0.75475421495049964964e-2 * t36127 - t37874 - t36132 - t36134 - t37877 - 0.62896184579208304136e-3 * t40436 + 0.39624596284901231605e-1 * t36137 - t37879 - t31751 - 0.13208198761633743869e-1 * t31752 - 0.21437009059034868486e-2 * t40442 + t36152 - t40446 / 128.0 - t36157;
    (t40448,)
}
