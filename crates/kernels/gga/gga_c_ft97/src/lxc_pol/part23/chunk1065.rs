//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1065/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1065<F: Float>(t2252: F, t2326: F, t342: F, t762: F, t9895: F, t2492: F, t2568: F, t9802: F, t192: F, t33300: F, t2469: F, t754: F, t70: F, t9651: F, t2404: F, t2680: F) -> (F, F, F, F, F, F, F, F, F) {
    let t42295 = t342 * t2252 * t2326;
    let t42334 = t9895 * t762;
    let t42339 = t2492 * t2568;
    let t42362 = t9802 * t762;
    let t42500 = t192 * t33300;
    let t42575 = t2492 * t2469;
    let t42939 = t9802 * t754;
    let t43194 = t70 * t9651;
    let t43350 = t2404 * t2680;
    (t42295, t42334, t42339, t42362, t42500, t42575, t42939, t43194, t43350)
}
