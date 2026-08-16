//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 865/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk865<F: Float>(t1415: F, t7030: F, t9297: F, t2372: F, t39776: F, t900: F, t2464: F, t2465: F, t2487: F, t9171: F, t20535: F, t29969: F, t4782: F, t883: F) -> (F, F, F, F) {
    let t40106 = t1415 * t9297 * t7030;
    let t40109 = t2372 * t900 * t39776;
    let t40116 = t2487 * t2464 * t2465 * t9171;
    let t40147 = t20535 * t4782 * t883 * t29969;
    (t40106, t40109, t40116, t40147)
}
