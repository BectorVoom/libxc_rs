//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 533/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk533<F: Float>(t2662: F, t4353: F, t2661: F, t1565: F, t2652: F, t1561: F, t2741: F, t241: F, t2719: F, t820: F, t243: F, t72: F) -> (F, F, F, F, F, F) {
    let t4354 = t2662 * t4353;
    let t4355 = t2661 * t4354;
    let t4357 = t2652 * t1565;
    let t4359 = t2741 * t1561;
    let t4362 = t820 * t2719 * t241;
    let t4363 = t243 * t72;
    (t4354, t4355, t4357, t4359, t4362, t4363)
}
