//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 695/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk695<F: Float>(t888: F, t9263: F, t9278: F, t1415: F, t7030: F, t9301: F, t30639: F, t6590: F, t12455: F, t18067: F, t12507: F, t4379: F, t1429: F, t2365: F, t2366: F, t9127: F) -> (F, F, F, F, F, F) {
    let t40225 = t9263 * t888 * t9278;
    let t40228 = t1415 * t9301 * t7030;
    let t40234 = t30639 * t6590;
    let t40237 = t18067 * t12455;
    let t40239 = t4379 * t12507;
    let t40243 = t1429 * t2365 * t2366 * t9127;
    (t40225, t40228, t40234, t40237, t40239, t40243)
}
