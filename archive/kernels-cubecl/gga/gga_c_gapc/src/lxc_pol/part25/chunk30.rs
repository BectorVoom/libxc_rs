//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 30/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk30<F: Float>(t93: F, t96: F, t62: F, t80: F, t85: F, t88: F) -> (F, F) {
    let t97 = t93 * t96;
    let t99 = -F::cast_from(0.59778596625315888114e-2_f64) * t62 + F::cast_from(0.1317375e-2_f64) * t80 - F::cast_from(0.23775e-3_f64) * t85 + F::cast_from(0.64744236347453835951e-5_f64) * t88 - F::cast_from(0.540140625e-6_f64) * t97;
    (t97, t99)
}
