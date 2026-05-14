//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 555/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk555<F: Float>(t129: F, t1692: F, t1685: F, t3097: F, t3091: F, t713: F, t928: F, t871: F, t931: F, t295: F, t3113: F, t2558: F, t954: F) -> (F, F, F, F, F, F, F) {
    let t3220 = t1692 * t129;
    let t3221 = t3097 * t1685;
    let t3222 = t3221 * M_PI;
    let t3223 = t3220 * t3222;
    let t3225 = t713 * t3091;
    let t3226 = t3225 * t928;
    let t3230 = t931 * t871;
    let t3232 = t295 * t3113;
    let t3240 = t954 * t2558;
    (t3220, t3223, t3225, t3226, t3230, t3232, t3240)
}
