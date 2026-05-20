//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1765/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1765<F: Float>(t11249: F, t1794: F, t3172: F, t5303: F, t1261: F, t1209: F, t489: F, t3623: F, t370: F) -> (F, F, F, F, F) {
    let t17710 = t1794 * t11249;
    let t17720 = t3172 * t5303;
    let t17721 = t1261 * t17720;
    let t17727 = t1209 * t489;
    let t17728 = t3623 * t370;
    (t17710, t17720, t17721, t17727, t17728)
}
