//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 568/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk568<F: Float>(t3641: F, t773: F, t11608: F, t701: F, t1445: F, t11604: F, t1457: F, t1: F, t3601: F, t106: F, t316: F) -> (F, F, F, F, F, F) {
    let t11743 = t773 * t3641;
    let t11748 = t11608 * t701;
    let t11749 = t1445 * t11748;
    let t11752 = t1457 * t11604;
    let t11755 = t3601 * t1;
    let t11756 = t11755 * t106;
    let t11757 = t11756 * t316;
    (t11743, t11749, t11752, t11755, t11756, t11757)
}
