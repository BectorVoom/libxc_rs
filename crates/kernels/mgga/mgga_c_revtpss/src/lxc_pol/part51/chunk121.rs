//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 121/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk121<F: Float>(t281: F, t282: F, t414: F, t406: F, t409: F, t412: F, t408: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t416 = t281 * t282 * t414;
    let t418 = 0.379785e1 * t409 + 0.8969e0 * t406 + 0.204775e0 * t412 + 0.123235e0 * t416;
    let t421 = 1.0 + 0.16081979498692535067e2 / t418;
    let t422 = f64::ln(t421);
    let t424 = 0.621814e-1 * t408 * t422;
    let t426 = 1.0 + 0.5137e-1 * t406;
    let t431 = 0.705945e1 * t409 + 0.1549425e1 * t406 + 0.420775e0 * t412 + 0.1562925e0 * t416;
    let t434 = 1.0 + 0.32163958997385070134e2 / t431;
    let t435 = f64::ln(t434);
    let t439 = 1.0 + 0.278125e-1 * t406;
    (t416, t418, t421, t422, t424, t426, t431, t434, t435, t439)
}
