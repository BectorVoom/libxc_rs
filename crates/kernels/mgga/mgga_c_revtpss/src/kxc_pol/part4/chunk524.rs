//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 524/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk524<F: Float>(t2455: F, t2457: F, t2454: F, t786: F, t861: F, t789: F, t252: F, t867: F) -> (F, F, F, F, F, F) {
    let t2458 = t2455 * t2457;
    let t2460 = 0.11565819519348392139e-2 * t2454 * t2458;
    let t2461 = t786 * t861;
    let t2462 = t2461 * t789;
    let t2464 = t252 * t867;
    let t2465 = t786 * t2464;
    (t2458, t2460, t2461, t2462, t2464, t2465)
}
