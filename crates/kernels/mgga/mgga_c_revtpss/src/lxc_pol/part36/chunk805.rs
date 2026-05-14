//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 805/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk805<F: Float>(t3434: F, t421: F, t3431: F, t418: F, t408: F, t240: F, t3698: F, t3361: F, t635: F, t57: F, t268: F, t404: F, t7021: F, t159: F, t3617: F, t409: F, t416: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12230 = 1.0 / t3434 / t421;
    let t12247 = 1.0 / t3431 / t418;
    let t12248 = t408 * t12247;
    let t12254 = t240 * t3698;
    let t12256 = 1.0 / t3361 / t635;
    let t12267 = t3361 * t57;
    let t12268 = 1.0 / t12267;
    let t12295 = t268 * t7021 * t404;
    let t12296 = 28.0 / 27.0 * t12295;
    let t12305 = t159 * t3617;
    let t12327 = 1.0 / t409 / t416 / 4.0;
    (t12230, t12248, t12254, t12256, t12268, t12295, t12296, t12305, t12327)
}
