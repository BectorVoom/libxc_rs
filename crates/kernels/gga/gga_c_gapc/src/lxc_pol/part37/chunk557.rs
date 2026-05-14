//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 557/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk557<F: Float>(t3403: F, t3408: F, t1084: F, t2995: F, t6: F, t966: F, t134: F, t875: F, t3405: F) -> (F, F, F, F, F, F) {
    let t3409 = t3403 * t3408;
    let t3411 = t1084 * t2995;
    let t3412 = t966 * t6;
    let t3413 = t134 * t875;
    let t3414 = t3412 * t3413;
    let t3415 = t3405 * t3414;
    (t3409, t3411, t3412, t3413, t3414, t3415)
}
