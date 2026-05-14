//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 30/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk30<F: Float>(t93: F, t96: F, t62: F, t80: F, t85: F, t88: F) -> (F, F) {
    let t97 = t93 * t96;
    let t99 = -0.59778596625315888114e-2 * t62 + 0.1317375e-2 * t80 - 0.23775e-3 * t85 + 0.64744236347453835951e-5 * t88 - 0.540140625e-6 * t97;
    (t97, t99)
}
