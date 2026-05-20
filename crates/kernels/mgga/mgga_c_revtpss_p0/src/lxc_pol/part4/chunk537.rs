//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 537/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk537<F: Float>(t2453: F, t252: F, t136: F, t257: F, t124: F, t137: F, t68: F) -> (F, F, F, F) {
    let t2454 = t2453 * t252;
    let t2455 = t257 * t136;
    let t2456 = t137 * t124;
    let t2457 = t2456 * t68;
    (t2454, t2455, t2456, t2457)
}
