//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1207/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1207<F: Float>(t101783: F, t101793: F, t108966: F, t108990: F, t110039: F, t110044: F, t114246: F, t114264: F, t114322: F, t114349: F, t2048: F, t26175: F, t28154: F, t28628: F, t29551: F, t7964: F, t95316: F) -> F {
    let t115291 = F::cast_from(20.0_f64) * t108966 * t28628 + F::cast_from(20.0_f64) * t28154 * t110039 + F::cast_from(30.0_f64) * t26175 * t114246 + F::cast_from(10.0_f64) * t28154 * t110044 + F::cast_from(10.0_f64) * t108990 * t28628 - F::cast_from(2.0_f64) * t29551 * t7964 - F::cast_from(2.0_f64) * t114322 * t2048 - F::cast_from(440.0_f64) / F::cast_from(9.0_f64) * t101783 - F::cast_from(176.0_f64) / F::cast_from(9.0_f64) * t101793 - F::cast_from(70.0_f64) * t95316 * t114264 + t114349 * t2048 / F::cast_from(3.0_f64);
    t115291
}
