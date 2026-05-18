//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 800/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk800<F: Float>(t191: F, t2153: F, t1093: F, t122: F, t2786: F, t2995: F, t3408: F, t3363: F, t3415: F, t1081: F, t2648: F, t2594: F, t9408: F) -> (F, F, F, F, F, F) {
    let t9560 = t2153 * t191;
    let t9561 = t9560 * t1093;
    let t9563 = t2786 * t122;
    let t9564 = t9563 * t2995;
    let t9565 = t9564 * t3408;
    let t9567 = t3363 * t2995;
    let t9568 = t9567 * t3415;
    let t9570 = t1081 * t2648;
    let t9572 = t9408 * t2594;
    (t9561, t9563, t9565, t9568, t9570, t9572)
}
