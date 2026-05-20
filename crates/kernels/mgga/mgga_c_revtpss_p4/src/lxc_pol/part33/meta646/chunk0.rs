//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2095/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2095<F: Float>(t104695: F, t13148: F, t104707: F, t1285: F, t12987: F, t7623: F, t5261: F, t1230: F, t29082: F, t29037: F, t3636: F, t5326: F) -> (F, F, F, F, F, F, F) {
    let t104715 = t13148 * t104695;
    let t104721 = t1285 * t104707;
    let t104727 = t12987 * t7623;
    let t104732 = t5261 * t7623;
    let t104739 = t1230 * t29082;
    let t104742 = t29037 * t3636;
    let t104752 = t5326 * t7623;
    (t104715, t104721, t104727, t104732, t104739, t104742, t104752)
}
