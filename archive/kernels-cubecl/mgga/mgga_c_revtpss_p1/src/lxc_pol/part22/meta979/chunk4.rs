//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3294/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3294<F: Float>(t18410: F, t9775: F, t18392: F, t221: F, t2674: F, t2675: F, t18615: F, t231: F, t243: F, t2661: F, t2662: F, t14923: F, t18478: F) -> (F, F, F, F) {
    let t62445 = t9775 * t18410;
    let t62453 = t2674 * t2675 * t221 * t18392;
    let t62458 = t2661 * t2662 * t243 * t18615 * t231;
    let t62460 = t14923 * t18478;
    (t62445, t62453, t62458, t62460)
}
