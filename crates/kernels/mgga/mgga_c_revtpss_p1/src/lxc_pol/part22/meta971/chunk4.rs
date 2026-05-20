//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3248/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3248<F: Float>(t18432: F, t40336: F, t5977: F, t853: F, t10726: F, t10786: F, t2661: F, t18495: F, t2652: F, t18500: F, t18493: F, t221: F) -> (F, F, F, F, F, F) {
    let t61623 = t40336 * t18432;
    let t61625 = t853 * t5977;
    let t61628 = t2661 * t10726 * t61625 * t10786;
    let t61630 = t2652 * t18495;
    let t61632 = t2652 * t18500;
    let t61639 = t221 * t18493;
    (t61623, t61625, t61628, t61630, t61632, t61639)
}
