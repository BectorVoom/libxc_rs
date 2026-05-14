//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1168/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1168<F: Float>(t1409: F, t7148: F, t1056: F, t3622: F, t3630: F, t2707: F, t9321: F, t7488: F, t9324: F, t7494: F, t221: F, t2631: F, t3636: F, t7491: F, t7485: F, t1100: F, t462: F, t9369: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t25826 = t7148 * t1409;
    let t25907 = 32.0 * t3622 * t1056;
    let t25930 = 32.0 * t3630 * t1056;
    let t25937 = t9321 * t2707;
    let t25939 = t9324 * t7488;
    let t25941 = t9324 * t7494;
    let t25944 = t3636 * t221 * t2631;
    let t25946 = t9324 * t7491;
    let t25948 = t9324 * t7485;
    let t25951 = t462 * t9369 * t1100;
    (t25826, t25907, t25930, t25937, t25939, t25941, t25944, t25946, t25948, t25951)
}
