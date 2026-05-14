//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 115/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk115<F: Float>(t301: F, t346: F, t345: F, t22: F, t37: F, t130: F, t11: F, t14: F) -> (F, F, F, F, F) {
    let t347 = t346 * t301;
    let t348 = t345 * t347;
    let t351 = 1.0 / t22 / t37;
    let t352 = t130 * t351;
    let t355 = 1.0 / t14 / t11;
    (t347, t348, t351, t352, t355)
}
