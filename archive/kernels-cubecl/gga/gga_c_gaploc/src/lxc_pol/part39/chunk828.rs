//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 828/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk828<F: Float>(t501: F, t9241: F, t5538: F, t883: F, t28668: F, t7290: F, t2547: F, t279: F, t481: F, t747: F, t9765: F, t1959: F, t3259: F) -> (F, F, F, F, F, F) {
    let t29096 = t9241 * t501;
    let t29277 = t5538 * t883;
    let t29285 = t7290 * t28668;
    let t29439 = t481 * t2547 * t279;
    let t29646 = t9765 * t747;
    let t29650 = t3259 * t1959;
    (t29096, t29277, t29285, t29439, t29646, t29650)
}
