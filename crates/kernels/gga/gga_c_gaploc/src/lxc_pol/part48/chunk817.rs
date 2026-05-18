//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 817/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk817<F: Float>(t2617: F, t3451: F, t7803: F, t13069: F, t7416: F, t10040: F, t25198: F, t11112: F, t2679: F, t9800: F, t13055: F, t5640: F) -> (F, F, F, F, F) {
    let t43609 = t7803 * t3451 * t2617;
    let t43611 = t7416 * t13069;
    let t43646 = t25198 * t10040;
    let t43650 = t9800 * t11112 * t2679;
    let t43652 = t5640 * t13055;
    (t43609, t43611, t43646, t43650, t43652)
}
