//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1114/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1114<F: Float>(t108280: F, t27762: F, t6118: F, t743: F, t9568: F, t108395: F, t108401: F, t27805: F, t108439: F, t24432: F, t108366: F, t97181: F, t192: F, t9942: F, t24437: F, t24477: F, t6852: F) -> (F, F, F, F, F, F) {
    let t109375 = t6118 * t27762 * t108280;
    let t109377 = t9568 * t743;
    let t109379 = t6118 * t109377 * t108395;
    let t109382 = t27805 * t27762 * t108401;
    let t109385 = t6118 * t24432 * t108439;
    let t109388 = t6118 * t97181 * t108366;
    let t109390 = t192 * t9942;
    let t109393 = t24437 * t109390 * t6852 * t24477;
    (t109375, t109379, t109382, t109385, t109388, t109393)
}
