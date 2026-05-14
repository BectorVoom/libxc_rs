//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 466/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk466<F: Float>(t1651: F, t996: F, t1015: F, t1469: F, t1012: F, t1647: F, t225: F) -> (F, F, F, F) {
    let t1652 = t996 * t1651;
    let t1655 = t1015 * t1469;
    let t1656 = t1012 * t1655;
    let t1659 = t1647 * t225;
    (t1652, t1655, t1656, t1659)
}
