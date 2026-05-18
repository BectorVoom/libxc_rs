//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 786/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk786<F: Float>(t12507: F, t4379: F, t1429: F, t2365: F, t2366: F, t9127: F, t12538: F, t1407: F, t2464: F, t2465: F, t587: F, t9316: F) -> (F, F, F, F) {
    let t40239 = t4379 * t12507;
    let t40243 = t1429 * t2365 * t2366 * t9127;
    let t40245 = t1407 * t12538;
    let t40249 = t587 * t2464 * t2465 * t9316;
    (t40239, t40243, t40245, t40249)
}
