//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 805/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk805<F: Float>(t301: F, t30407: F, t30408: F, t30409: F, t30402: F, t360: F, t172: F, t2066: F) -> (F, F, F) {
    let t30412 = t30407 * t30408 * t30409 * t301;
    let t30416 = t30407 * t30402 * t30409 * t360;
    let t30418 = t2066 * t172;
    (t30412, t30416, t30418)
}
