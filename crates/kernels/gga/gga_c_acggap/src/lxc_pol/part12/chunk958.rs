//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 958/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk958<F: Float>(t1345: F, t1983: F, t7380: F, t1462: F, t7614: F, t1446: F, t7605: F, t1441: F, t1456: F, t1998: F, t4720: F, t1298: F, t7381: F, t1524: F, t2095: F, t435: F, t7815: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t35390 = t7380 * t1983 * t1345;
    let t35392 = t7614 * t1462;
    let t35394 = t7605 * t1446;
    let t35396 = t7605 * t1441;
    let t35398 = t7605 * t1456;
    let t35400 = t7605 * t1462;
    let t35403 = t1998 * t4720;
    let t35407 = t7380 * t7381 * t1298;
    let t35410 = t2095 * t1983 * t1524;
    let t35413 = t7815 * t435;
    (t35390, t35392, t35394, t35396, t35398, t35400, t35403, t35407, t35410, t35413)
}
