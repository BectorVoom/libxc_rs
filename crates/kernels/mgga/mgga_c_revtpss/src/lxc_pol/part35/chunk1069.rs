//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1069/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1069<F: Float>(t109892: F, t109983: F, t109985: F, t109988: F, t109990: F, t110008: F, t110010: F, t110014: F, t114343: F, t1923: F, t2047: F, t29513: F, t30543: F, t7702: F, t7964: F, t95253: F) -> (F,) {
    let t115305 = t29513 * t7964 + t7702 * t30543 + t1923 * t2047 * t114343 / 3.0 - 160.0 / 3.0 * t109892 - t95253 - 8.0 / 3.0 * t109983 - 16.0 / 3.0 * t109985 - 8.0 / 3.0 * t109988 + 16.0 / 3.0 * t109990 + 80.0 / 3.0 * t110008 + 32.0 / 3.0 * t110010 + 80.0 / 3.0 * t110014;
    (t115305,)
}
