//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 437/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk437<F: Float>(t2455: F, t2457: F, t2454: F, t786: F, t861: F, t789: F, t252: F, t867: F, t676: F, t886: F, t123: F, t215: F, t685: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2458 = t2455 * t2457;
    let t2460 = F::new(0.11565819519348392139e-2) * t2454 * t2458;
    let t2461 = t786 * t861;
    let t2462 = t2461 * t789;
    let t2464 = t252 * t867;
    let t2465 = t786 * t2464;
    let t2466 = t676 * t886;
    let t2467 = t123 * t2466;
    let t2468 = t2465 * t2467;
    let t2470 = t685 * t215;
    (t2458, t2460, t2461, t2462, t2464, t2465, t2466, t2467, t2468, t2470)
}
