//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1266/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1266<F: Float>(t3965: F, t9323: F, t14460: F, t4414: F, t1192: F, t13911: F, t19631: F, t22379: F, t2408: F, t2409: F, t3066: F, t3067: F, t4155: F, t51524: F, t53572: F, t53578: F, t53579: F, t53581: F, t53584: F, t53585: F, t53595: F, t53598: F, t53599: F, t9688: F) -> F {
    let t53601 = t3965 * t9323;
    let t53610 = F::new(7.0) / F::new(72.0) * t4414 * t14460;
    let t53613 = -t53572 / F::new(24.0) - t53578 - t53579 / F::new(48.0) - t53581 / F::new(48.0) - t53584 + F::new(35.0) / F::new(216.0) * t53585 + t2408 * t2409 * t19631 * t4155 / F::new(48.0) - F::new(5.0) / F::new(128.0) * t53595 - t53598 + t53599 / F::new(24.0) + t53601 / F::new(48.0) + t3066 * t2409 * t3067 * t1192 * t9688 / F::new(48.0) - F::new(7.0) / F::new(144.0) * t51524 - t53610 + t22379 * t13911 / F::new(24.0);
    t53613
}
