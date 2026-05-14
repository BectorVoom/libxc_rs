//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1292/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1292<F: Float>(t11003: F, t2598: F, t1006: F, t1014: F, t2576: F, t260: F, t2602: F, t29750: F, t29792: F, t29996: F, t29999: F, t30002: F, t30041: F, t30194: F, t3591: F, t3596: F, t3605: F, t4323: F, t7108: F, t8965: F, t8968: F, t9196: F, t9285: F) -> (F,) {
    let t30282 = t2598 * t11003;
    let t30297 = t29750 - t29792 + 0.2077903092681775651e3 * t3591 * t8965 + 0.23392894490538584828e1 * t1014 * t2576 * t30041 * t1006 - 0.34631718211362927517e2 * t3591 * t9285 - 0.34631718211362927518e2 * t1014 * t30282 * t3605 - 0.70178683471615754484e1 * t3591 * t8968 + 0.23392894490538584828e1 * t1014 * t3596 * t9196 - t29996 - t29999 + t30002 + 0.19751673498613801407e-1 * t260 * t30194 + 0.10389515463408878255e3 * t1014 * t7108 * t4323 * t2602;
    (t30297,)
}
