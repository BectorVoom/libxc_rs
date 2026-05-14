//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1069/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1069<F: Float>(t27819: F, t3938: F, t6119: F, t729: F, t747: F, t108220: F, t108224: F, t108229: F, t108233: F, t96953: F, t96958: F, t97320: F, t97324: F, t97327: F, t97329: F, t97333: F) -> (F, F) {
    let t108238 = t27819 * t729 * t6119 * t3938 * t747;
    let t108242 = -2.0 / 3.0 * t108220 + t97320 + t108224 / 3.0 - t108229 / 2.0 - 3.0 / 8.0 * t108233 - 3.0 / 4.0 * t108238 - t97324 + t97327 + 8.0 / 27.0 * t96953 - t97329 - 4.0 / 9.0 * t96958 - t97333;
    (t108238, t108242)
}
