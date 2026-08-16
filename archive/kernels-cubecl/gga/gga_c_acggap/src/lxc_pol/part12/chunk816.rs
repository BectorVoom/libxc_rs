//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 816/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk816<F: Float>(t467: F, t560: F, t1427: F, t8034: F, t5439: F, t8040: F, t104: F, t2407: F, t1614: F, t2176: F, t1410: F, t157: F, t2152: F, t633: F) -> (F, F, F, F, F, F) {
    let t9098 = t560 * t467;
    let t9108 = t8034 * t1427;
    let t9114 = t8040 * t5439;
    let t9121 = t104 * t2407;
    let t9129 = t2176 * t1614;
    let t9136 = t2152 * t633 * t1410 * t157;
    (t9098, t9108, t9114, t9121, t9129, t9136)
}
