//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 973/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk973<F: Float>(t28: F, t30266: F, t89: F, t4714: F, t5778: F, t23400: F, t4668: F, t24034: F, t24041: F, t27135: F, t27163: F, t27171: F, t27179: F, t30235: F, t30237: F, t30242: F, t30247: F, t30252: F, t30257: F, t30264: F) -> (F, F, F, F, F, F) {
    let t30267 = t28 * t30266;
    let t30268 = t89 * t30267;
    let t30270 = t5778 * t4714;
    let t30271 = t28 * t30270;
    let t30272 = t89 * t30271;
    let t30274 = t23400 * t4668;
    let t30275 = t28 * t30274;
    let t30276 = t89 * t30275;
    let t30279 = -t30235 - 4.0 / 9.0 * t30237 + t30242 / 3.0 + 2.0 / 3.0 * t30247 + t30252 / 12.0 + t30257 / 6.0 - t24034 - t27135 / 18.0 - t24041 - t27163 / 27.0 + 2.0 / 9.0 * t27171 - t30264 / 3.0 + 4.0 / 3.0 * t30268 + 2.0 / 3.0 * t30272 - 2.0 * t30276 - 4.0 / 9.0 * t27179;
    (t30268, t30270, t30272, t30274, t30276, t30279)
}
