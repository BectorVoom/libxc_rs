//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3048/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3048<F: Float>(t14593: F, t2470: F, t874: F, t1558: F, t2482: F, t2801: F, t2815: F, t10547: F, t14606: F, t10538: F, t14605: F, t49180: F) -> (F, F, F, F) {
    let t51587 = t874 * t14593 * t2470;
    let t51598 = t2482 * t2815 * t1558 * t2801;
    let t51600 = t14606 * t10547;
    let t51603 = t49180 * t14605 * t10538;
    (t51587, t51598, t51600, t51603)
}
