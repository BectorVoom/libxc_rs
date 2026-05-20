//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3165/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3165<F: Float>(t17769: F, t3647: F, t1235: F, t371: F, t5318: F, t676: F, t225: F, t56331: F, t1789: F, t2434: F, t1261: F, t16746: F, t247: F, t3634: F) -> (F, F, F, F, F) {
    let t57451 = t3647 * t17769;
    let t57463 = t1235 * t371 * t676 * t5318;
    let t57465 = t56331 * t225;
    let t57471 = t1235 * t371 * t2434 * t1789;
    let t57478 = t1261 * t247 * t3634 * t16746;
    (t57451, t57463, t57465, t57471, t57478)
}
