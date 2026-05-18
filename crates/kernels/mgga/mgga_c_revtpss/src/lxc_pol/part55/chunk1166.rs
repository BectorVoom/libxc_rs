//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1166/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1166<F: Float>(t2118: F, t7690: F, t2110: F, t7700: F, t2167: F, t7560: F, t1455: F, t8909: F, t33316: F, t575: F, t33338: F, t571: F) -> (F, F, F, F, F, F) {
    let t124413 = t7690 * t2118;
    let t124418 = t2110 * t7700;
    let t124420 = t2167 * t7560;
    let t124429 = t1455 * t8909;
    let t124431 = t33316 * t575;
    let t124435 = t571 * t33338;
    (t124413, t124418, t124420, t124429, t124431, t124435)
}
