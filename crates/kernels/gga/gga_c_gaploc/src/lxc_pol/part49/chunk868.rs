//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 868/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk868<F: Float>(t40190: F, t587: F, t912: F, t2464: F, t2465: F, t9198: F, t29975: F, t31119: F, t31120: F, t883: F, t2482: F, t9272: F, t9354: F) -> (F, F, F, F) {
    let t40192 = t587 * t912 * t40190;
    let t40196 = t587 * t2464 * t2465 * t9198;
    let t40202 = t31119 * t31120 * t883 * t29975;
    let t40208 = t9272 * t9354 * t2482;
    (t40192, t40196, t40202, t40208)
}
