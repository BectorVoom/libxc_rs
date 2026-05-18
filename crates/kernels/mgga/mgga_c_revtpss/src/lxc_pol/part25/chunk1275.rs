//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1275/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1275<F: Float>(t1024: F, t25576: F, t25525: F, t3123: F, t11997: F, t3141: F, t7120: F, t11858: F, t27492: F, t11926: F, t25516: F, t3114: F, t93596: F) -> (F, F, F, F, F, F) {
    let t93646 = t1024 * t25576;
    let t93649 = t3123 * t25525;
    let t93655 = t3141 * t7120 * t11997;
    let t93658 = t11858 * t27492;
    let t93667 = t11926 * t25516;
    let t93670 = t3114 * t93596;
    (t93646, t93649, t93655, t93658, t93667, t93670)
}
