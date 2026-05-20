//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2720/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2720<F: Float>(t21063: F, t3678: F, t17225: F, t5381: F, t1261: F, t20791: F, t3172: F, t13058: F, t20786: F, t11262: F, t3711: F, t6618: F) -> (F, F, F, F, F) {
    let t70265 = t21063 * t3678;
    let t70270 = t5381 * t17225;
    let t70273 = t1261 * t3172 * t20791;
    let t70275 = t13058 * t20786;
    let t70278 = t3711 * t11262 * t6618;
    (t70265, t70270, t70273, t70275, t70278)
}
