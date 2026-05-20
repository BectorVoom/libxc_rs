//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2606/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2606<F: Float>(t2661: F, t2662: F, t50583: F, t6035: F, t18408: F, t837: F, t18432: F, t40336: F, t5977: F, t853: F, t10726: F, t10786: F) -> (F, F, F, F, F) {
    let t61616 = t2661 * t2662 * t50583 * t6035;
    let t61620 = t2661 * t2662 * t18408 * t837;
    let t61623 = t40336 * t18432;
    let t61625 = t853 * t5977;
    let t61628 = t2661 * t10726 * t61625 * t10786;
    (t61616, t61620, t61623, t61625, t61628)
}
