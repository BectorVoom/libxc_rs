//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3190/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3190<F: Float>(t12901: F, t17572: F, t17235: F, t372: F, t13068: F, t5292: F, t1032: F, t1246: F, t17331: F, t1247: F, t17221: F, t3172: F) -> (F, F, F, F, F) {
    let t59360 = t17572 * t12901;
    let t59362 = t372 * t17235;
    let t59371 = t13068 * t5292;
    let t59375 = t17331 * t1032 * t1246;
    let t59379 = t1247 * t3172 * t17221;
    (t59360, t59362, t59371, t59375, t59379)
}
