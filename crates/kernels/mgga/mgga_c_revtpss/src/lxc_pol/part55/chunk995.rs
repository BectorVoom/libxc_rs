//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 995/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk995<F: Float>(t225: F, t29109: F, t494: F, t1769: F, t7627: F, t7637: F, t11239: F, t1276: F, t3596: F, t2149: F, t29157: F, t3153: F) -> (F, F, F, F, F) {
    let t29183 = t29109 * t225 * t494;
    let t29186 = t7627 * t1769;
    let t29187 = t7637 * t29186;
    let t29192 = t11239 * t1276;
    let t29193 = t29192 * t3596;
    let t29194 = t2149 * t29193;
    let t29195 = t29157 * t3153;
    (t29183, t29187, t29192, t29194, t29195)
}
