//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1291/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1291<F: Float>(t29083: F, t5378: F, t21090: F, t26867: F, t29019: F, t5273: F, t20973: F, t7624: F, t1785: F, t29082: F, t21192: F, t1219: F, t30800: F) -> (F, F, F, F, F, F, F) {
    let t112234 = t29083 * t5378;
    let t112243 = t26867 * t21090;
    let t112252 = t5273 * t29019;
    let t112258 = t7624 * t20973;
    let t112260 = t1785 * t29082;
    let t112279 = t7624 * t21192;
    let t112301 = t30800 * t1219;
    (t112234, t112243, t112252, t112258, t112260, t112279, t112301)
}
