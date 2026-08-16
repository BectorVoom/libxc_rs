//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2022/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2022<F: Float>(t110378: F, t110385: F, t110393: F, t110406: F, t110414: F, t110421: F, t110429: F, t110441: F, t27216: F, t28360: F, t30384: F, t786: F, t789: F) -> (F, F, F) {
    let t110444 = t110378 + t110385 + t110393 + t110406 + t110414 + t110421 + t110429 + t110441;
    let t110453 = t27216 * t28360;
    let t110459 = t786 * t30384 * t789;
    (t110444, t110453, t110459)
}
