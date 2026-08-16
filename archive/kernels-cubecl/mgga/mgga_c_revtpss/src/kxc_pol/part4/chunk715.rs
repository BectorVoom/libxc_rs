//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 715/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk715<F: Float>(t3617: F, t66: F, t3363: F, t247: F, t474: F, t479: F) -> (F, F, F) {
    let t3618 = t66 * t3617;
    let t3619 = t3618 * t3363;
    let t3620 = t247 * t3619;
    let t3623 = t474 * t479;
    (t3618, t3620, t3623)
}
