//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 687/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk687<F: Float>(t1211: F, t3584: F, t3378: F, t3381: F, t3388: F, t3430: F, t3438: F, t3528: F, t3530: F, t3533: F, t3537: F, t3541: F, t3545: F) -> (F, F) {
    let t3585 = t1211 * t3584;
    let t3588 = -t3378 + t3381 - t3388 + t3430 + t3438 + t3528 + t3530 - t3533 + t3537 - t3541 - t3545;
    (t3585, t3588)
}
