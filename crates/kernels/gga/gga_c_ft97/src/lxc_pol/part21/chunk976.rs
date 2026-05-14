//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 976/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk976<F: Float>(t23899: F, t23924: F, t27135: F, t27163: F, t27171: F, t27179: F, t30235: F, t30237: F, t30242: F, t30247: F, t30252: F, t30257: F, t30264: F, t30268: F, t30272: F, t30276: F) -> (F,) {
    let t30356 = -3.0 * t30235 - 4.0 / 3.0 * t30237 + t30242 + 2.0 * t30247 + t30252 / 4.0 + t30257 / 2.0 - t23899 - t27135 / 6.0 - t23924 - t27163 / 9.0 + 2.0 / 3.0 * t27171 - t30264 + 4.0 * t30268 + 2.0 * t30272 - 6.0 * t30276 - 4.0 / 3.0 * t27179;
    (t30356,)
}
