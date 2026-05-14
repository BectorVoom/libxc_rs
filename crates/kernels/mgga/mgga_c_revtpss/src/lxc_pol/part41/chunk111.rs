//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 111/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk111<F: Float>(t281: F, t282: F, t283: F, t273: F, t276: F, t279: F) -> (F, F, F, F) {
    let t285 = t281 * t282 * t283;
    let t287 = 0.379785e1 * t276 + 0.8969e0 * t273 + 0.204775e0 * t279 + 0.123235e0 * t285;
    let t290 = 1.0 + 0.16081979498692535067e2 / t287;
    let t291 = f64::ln(t290);
    (t285, t287, t290, t291)
}
