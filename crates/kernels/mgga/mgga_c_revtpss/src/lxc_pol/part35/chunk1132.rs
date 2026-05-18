//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1132/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1132<F: Float>(t25877: F, t97699: F, t25304: F, t27883: F, t25898: F, t2453: F, t1955: F, t27836: F, t4075: F, t1892: F, t7063: F, t27928: F, t9775: F) -> (F, F, F, F, F, F, F, F) {
    let t97700 = t97699 * t25877;
    let t97799 = t25304 * t27883;
    let t97802 = t97699 * t25898;
    let t97916 = t2453 * t27883;
    let t97933 = t1955 * t27836 * t4075;
    let t98040 = t7063 * t1892;
    let t98041 = t98040 * t25877;
    let t98141 = t9775 * t27928;
    (t97700, t97799, t97802, t97916, t97933, t98040, t98041, t98141)
}
