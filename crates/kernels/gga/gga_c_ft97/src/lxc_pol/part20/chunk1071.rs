//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1071/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1071<F: Float>(t10157: F, t14053: F, t6118: F, t6119: F, t108247: F, t108250: F, t108253: F, t108258: F, t108261: F, t108263: F, t108266: F, t96983: F, t97335: F, t97339: F, t97344: F, t14213: F) -> (F, F, F) {
    let t108270 = t6118 * t10157 * t6119 * t14053;
    let t108272 = t97335 + 4.0 / 27.0 * t96983 + t97339 - t97344 + t108247 / 9.0 - t108250 + t108253 / 6.0 + t108258 / 3.0 + t108261 + t108263 - 6.0 * t108266 - 6.0 * t108270;
    let t108275 = t6118 * t10157 * t6119 * t14213;
    (t108270, t108272, t108275)
}
