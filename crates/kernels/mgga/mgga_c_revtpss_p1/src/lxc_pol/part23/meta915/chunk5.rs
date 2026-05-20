//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2954/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2954<F: Float>(t300: F, t77637: F, t77873: F, t78155: F, t78196: F, t78240: F, t78279: F, t78316: F, t78398: F, t77492: F, t77494: F, t77496: F, t77498: F, t77600: F, t77604: F, t77612: F, t77622: F, t77624: F, t77628: F) -> (F, F) {
    let t78402 = t300 * (t77637 + t77873 + t78155 + t78196 + t78240 + t78279 + t78316 + t78398);
    let t78403 = -t77492 - t77494 - t77496 - t77498 + t77600 - t77604 + t78402 - t77612 + t77622 + t77624 + t77628;
    (t78402, t78403)
}
