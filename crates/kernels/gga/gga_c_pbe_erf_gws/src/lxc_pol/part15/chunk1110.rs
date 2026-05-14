//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1110/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1110<F: Float>(t3974: F, t3990: F, t53592: F, t8939: F, t14602: F, t51666: F, t3959: F, t9704: F, t3965: F, t9323: F, t14460: F, t4414: F, t1192: F, t13911: F, t19631: F, t22379: F, t2408: F, t2409: F, t3066: F, t3067: F, t4155: F, t51524: F, t53572: F, t53578: F, t53579: F, t53581: F, t53584: F, t53585: F, t9688: F) -> (F,) {
    let t53595 = t53592 * t3990 * t3974 * t8939;
    let t53597 = t51666 * t14602;
    let t53598 = 7.0 / 576.0 * t53597;
    let t53599 = t3959 * t9704;
    let t53601 = t3965 * t9323;
    let t53610 = 7.0 / 72.0 * t4414 * t14460;
    let t53613 = -t53572 / 24.0 - t53578 - t53579 / 48.0 - t53581 / 48.0 - t53584 + 35.0 / 216.0 * t53585 + t2408 * t2409 * t19631 * t4155 / 48.0 - 5.0 / 128.0 * t53595 - t53598 + t53599 / 24.0 + t53601 / 48.0 + t3066 * t2409 * t3067 * t1192 * t9688 / 48.0 - 7.0 / 144.0 * t51524 - t53610 + t22379 * t13911 / 24.0;
    (t53613,)
}
