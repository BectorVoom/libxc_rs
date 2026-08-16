//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 780/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk780<F: Float>(t7371: F, t7372: F, t5654: F, t824: F, t2618: F, t1902: F, t2465: F, t2464: F, t2615: F, t161: F, t165: F, t1710: F) -> (F, F, F, F, F) {
    let t7373 = t7371 * t7372;
    let t7375 = t5654 * t824;
    let t7376 = t7375 * t2618;
    let t7378 = t2465 * t1902;
    let t7379 = t2464 * t7378;
    let t7380 = t2615 * t7379;
    let t7383 = t161 * t165 * t1710;
    (t7373, t7375, t7376, t7380, t7383)
}
