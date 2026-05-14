//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 898/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk898<F: Float>(t4643: F, t7486: F, t2095: F, t1427: F, t31491: F, t7381: F, t1345: F, t1983: F, t7380: F, t1462: F, t7614: F, t1446: F, t7605: F, t1441: F, t1456: F, t1998: F, t4720: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t35383 = t4643 * t7486;
    let t35384 = t2095 * t35383;
    let t35387 = t31491 * t7381 * t1427;
    let t35390 = t7380 * t1983 * t1345;
    let t35392 = t7614 * t1462;
    let t35394 = t7605 * t1446;
    let t35396 = t7605 * t1441;
    let t35398 = t7605 * t1456;
    let t35400 = t7605 * t1462;
    let t35403 = t1998 * t4720;
    (t35383, t35384, t35387, t35390, t35392, t35394, t35396, t35398, t35400, t35403)
}
