//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1068/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1068<F: Float>(t101783: F, t101793: F, t108966: F, t108990: F, t110039: F, t110044: F, t114246: F, t114264: F, t114322: F, t114349: F, t2048: F, t26175: F, t28154: F, t28628: F, t29551: F, t7964: F, t95316: F) -> (F,) {
    let t115291 = 20.0 * t108966 * t28628 + 20.0 * t28154 * t110039 + 30.0 * t26175 * t114246 + 10.0 * t28154 * t110044 + 10.0 * t108990 * t28628 - 2.0 * t29551 * t7964 - 2.0 * t114322 * t2048 - 440.0 / 9.0 * t101783 - 176.0 / 9.0 * t101793 - 70.0 * t95316 * t114264 + t114349 * t2048 / 3.0;
    (t115291,)
}
