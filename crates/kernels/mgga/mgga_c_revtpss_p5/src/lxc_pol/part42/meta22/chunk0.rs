//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 145/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk145<F: Float>(t281: F, t282: F, t414: F, t406: F, t409: F, t412: F) -> (F, F, F, F) {
    let t416 = t281 * t282 * t414;
    let t418 = F::new(0.379785e1) * t409 + F::new(0.8969e0) * t406 + F::new(0.204775e0) * t412 + F::new(0.123235e0) * t416;
    let t421 = F::new(1.0) + F::cast_from(0.16081979498692535067e2_f64) / t418;
    let t422 = F::ln(t421);
    (t416, t418, t421, t422)
}
