//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1179/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1179<F: Float>(t126030: F, t25207: F, t27363: F, t30: F, t119763: F, t1561: F, t1558: F, t257: F, t119767: F, t247: F, t2749: F, t119757: F, t31846: F, t4451: F) -> (F, F, F, F, F, F) {
    let t126031 = t25207 * t126030;
    let t126037 = t30 * t27363;
    let t126043 = t119763 * t1561;
    let t126046 = t257 * t1558;
    let t126049 = t119767 * t247 * t126046 * t2749;
    let t126052 = t31846 * t119757 * t4451;
    (t126031, t126037, t126043, t126046, t126049, t126052)
}
