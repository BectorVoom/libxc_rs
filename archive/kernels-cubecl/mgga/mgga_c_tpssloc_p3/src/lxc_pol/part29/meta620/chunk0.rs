//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2062/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2062<F: Float>(t24574: F, t24860: F, t24594: F, t24847: F, t974: F, t27551: F, t7327: F, t135: F, t7284: F, t24853: F, t24778: F, t24762: F) -> (F, F, F, F, F, F, F) {
    let t86073 = t24574 * t24860;
    let t86076 = t24847 * t974 * t24594;
    let t86077 = t7327 * t27551;
    let t86094 = t24847 * t135 * t7284;
    let t86095 = t86094 * t24853;
    let t86106 = t24574 * t24778;
    let t86113 = t24574 * t24762;
    (t86073, t86076, t86077, t86094, t86095, t86106, t86113)
}
