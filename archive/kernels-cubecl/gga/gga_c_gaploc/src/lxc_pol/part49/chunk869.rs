//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 869/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk869<F: Float>(t2478: F, t3125: F, t6576: F, t888: F, t9263: F, t9278: F, t1415: F, t7030: F, t9301: F, t30639: F, t6590: F, t12455: F, t18067: F) -> (F, F, F, F, F) {
    let t40219 = t6576 * t3125 * t2478;
    let t40225 = t9263 * t888 * t9278;
    let t40228 = t1415 * t9301 * t7030;
    let t40234 = t30639 * t6590;
    let t40237 = t18067 * t12455;
    (t40219, t40225, t40228, t40234, t40237)
}
