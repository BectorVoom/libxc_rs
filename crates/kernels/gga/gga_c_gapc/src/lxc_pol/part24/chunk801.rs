//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 801/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk801<F: Float>(t122: F, t2674: F, t2995: F, t134: F, t2254: F, t941: F, t3405: F, t3297: F, t9552: F, t2580: F, t9166: F, t2578: F) -> (F, F, F, F, F) {
    let t9574 = t2674 * t122;
    let t9575 = t9574 * t2995;
    let t9576 = t134 * t2254;
    let t9577 = t941 * t9576;
    let t9578 = t3405 * t9577;
    let t9579 = t9575 * t9578;
    let t9581 = t9552 * t3297;
    let t9583 = t9166 * t2580;
    let t9584 = t2578 * t9583;
    (t9574, t9578, t9579, t9581, t9584)
}
