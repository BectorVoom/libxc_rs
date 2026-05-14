//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 974/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk974<F: Float>(t762: F, t9802: F, t2542: F, t737: F, t192: F, t33300: F, t2469: F, t2492: F, t70: F, t9651: F, t2832: F, t2842: F, t2404: F, t2680: F, t683: F, t7640: F) -> (F, F, F, F, F, F, F, F, F) {
    let t42362 = t9802 * t762;
    let t42385 = t737 * t2542;
    let t42500 = t192 * t33300;
    let t42575 = t2492 * t2469;
    let t42996 = t2492 * t2542;
    let t43194 = t70 * t9651;
    let t43328 = t2832 * t2842;
    let t43350 = t2404 * t2680;
    let t43381 = t683 * t7640;
    (t42362, t42385, t42500, t42575, t42996, t43194, t43328, t43350, t43381)
}
