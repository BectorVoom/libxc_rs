//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2121/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2121<F: Float>(t98134: F, t98158: F, t98184: F, t98208: F, t98233: F, t98255: F, t98271: F, t98287: F, t543: F, t97870: F, t27857: F, t689: F) -> (F, F, F) {
    let t98290 = t98134 + t98158 + t98184 + t98208 + t98233 + t98255 + t98271 + t98287;
    let t98299 = t97870 * t543;
    let t98303 = t27857 * t689;
    (t98290, t98299, t98303)
}
