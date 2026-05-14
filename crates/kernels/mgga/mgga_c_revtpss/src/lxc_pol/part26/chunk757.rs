//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 757/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk757<F: Float>(t2609: F, t606: F, t706: F, t10592: F, t10594: F, t10596: F, t10598: F, t10602: F, t10604: F, t10607: F, t10609: F, t10611: F, t9542: F, t10550: F, t10571: F, t10590: F, t225: F) -> (F, F) {
    let t10612 = t2609 * t606;
    let t10613 = t706 * t10612;
    let t10614 = 12.0 * t10613;
    let t10615 = t10592 - t10594 - t10596 - t10598 + t10602 - t10604 + t9542 + t10607 + t10609 - t10611 + t10614;
    let t10618 = (t10550 + t10571 + t10590 + t10615) * t225;
    (t10614, t10618)
}
