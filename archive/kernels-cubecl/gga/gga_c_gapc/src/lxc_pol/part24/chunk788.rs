//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 788/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk788<F: Float>(t2600: F, t9408: F, t8769: F, t933: F, t2629: F, t1084: F, t8986: F, t2562: F, t2636: F, t8619: F, t3327: F, t7191: F) -> (F, F, F, F, F, F, F) {
    let t9409 = t9408 * t2600;
    let t9411 = t933 * t8769;
    let t9412 = t9411 * t2629;
    let t9414 = t1084 * t8986;
    let t9415 = t2636 * t2562;
    let t9416 = t9414 * t9415;
    let t9418 = t1084 * t8619;
    let t9419 = t3327 * t7191;
    (t9409, t9412, t9414, t9415, t9416, t9418, t9419)
}
