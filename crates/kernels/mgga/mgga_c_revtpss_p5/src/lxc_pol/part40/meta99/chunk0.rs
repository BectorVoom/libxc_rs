//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 543/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk543<F: Float>(t2339: F, t2340: F, t613: F, t99: F, t658: F, t100: F, t2256: F, t107: F, tau0: F) -> (F, F, F, F, F, F, F) {
    let t2341 = t2339 * t2340;
    let t2344 = tau0 * t613;
    let t2349 = F::cast_from(1.0_f64) / t99;
    let t2350 = t658 * t658;
    let t2351 = t2349 * t2350;
    let t2354 = t100 * t2256;
    let t2357 = F::cast_from(1.0_f64) / t107;
    (t2341, t2344, t2349, t2350, t2351, t2354, t2357)
}
