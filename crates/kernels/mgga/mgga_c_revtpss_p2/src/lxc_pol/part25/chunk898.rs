//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 898/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk898<F: Float>(t10550: F, t10571: F, t10590: F, t10615: F, t225: F, t2475: F, t73: F, t2394: F, t775: F) -> (F, F, F) {
    let t10618 = (t10550 + t10571 + t10590 + t10615) * t225;
    let t10626 = t73 * t2475;
    let t10627 = t2394 * t775;
    (t10618, t10626, t10627)
}
