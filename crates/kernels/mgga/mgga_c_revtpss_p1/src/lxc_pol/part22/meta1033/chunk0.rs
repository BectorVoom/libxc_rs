//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3616/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3616<F: Float>(t20343: F, t698: F, t20346: F, t141: F, t3417: F, t68355: F, t12254: F, t68340: F, t1134: F, t5079: F, t16851: F, t16854: F) -> (F, F, F, F, F, F) {
    let t68548 = t698 * t20343;
    let t68550 = t698 * t20346;
    let t68553 = t141 * t3417 * t68355;
    let t68556 = t141 * t12254 * t68340;
    let t68558 = t1134 * t5079;
    let t68559 = t16851 * t68558;
    let t68561 = t16854 * t68558;
    (t68548, t68550, t68553, t68556, t68559, t68561)
}
