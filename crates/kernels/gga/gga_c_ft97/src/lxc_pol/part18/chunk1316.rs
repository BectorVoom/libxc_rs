//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1316/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1316<F: Float>(t1359: F, t40465: F, t12590: F, t1901: F, t27083: F, t95053: F, t2112: F, t24: F, t23658: F, t27157: F, t6656: F, t1969: F, t23652: F, t27142: F, t3052: F, t3188: F, t95384: F) -> (F, F, F, F, F, F) {
    let t105429 = t40465 * t1359;
    let t105431 = t1901 * t105429 * t12590;
    let t105433 = t95053 * t27083;
    let t105434 = t105433 / 18.0;
    let t105435 = t24 * t2112;
    let t105438 = t27157 * t105435 * t6656 * t23658;
    let t105442 = t27142 * t1969 * t23652 * t3052;
    let t105444 = t95384 * t3188;
    (t105431, t105433, t105434, t105438, t105442, t105444)
}
