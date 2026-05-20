//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3684/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3684<F: Float>(t12050: F, t20956: F, t1261: F, t12879: F, t247: F, t6429: F, t11262: F, t1247: F, t6624: F, t21102: F, t3704: F, t17376: F, t17524: F) -> (F, F, F, F, F) {
    let t69655 = t20956 * t12050;
    let t69661 = t1261 * t247 * t12879 * t6429;
    let t69668 = t1247 * t11262 * t6624;
    let t69674 = t21102 * t3704;
    let t69680 = t17376 * t17524;
    (t69655, t69661, t69668, t69674, t69680)
}
