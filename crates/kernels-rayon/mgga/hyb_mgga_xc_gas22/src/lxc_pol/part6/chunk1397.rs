//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1397/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1397(t11003: f64, t2598: f64, t1006: f64, t1014: f64, t2576: f64, t260: f64, t2602: f64, t29750: f64, t29792: f64, t29996: f64, t29999: f64, t30002: f64, t30041: f64, t30194: f64, t3591: f64, t3596: f64, t3605: f64, t4323: f64, t7108: f64, t8965: f64, t8968: f64, t9196: f64, t9285: f64) -> f64 {
    let t30282 = t2598 * t11003;
    let t30297 = t29750 - t29792 + 0.2077903092681775651e3_f64 * t3591 * t8965 + 0.23392894490538584828e1_f64 * t1014 * t2576 * t30041 * t1006 - 0.34631718211362927517e2_f64 * t3591 * t9285 - 0.34631718211362927518e2_f64 * t1014 * t30282 * t3605 - 0.70178683471615754484e1_f64 * t3591 * t8968 + 0.23392894490538584828e1_f64 * t1014 * t3596 * t9196 - t29996 - t29999 + t30002 + 0.19751673498613801407e-1_f64 * t260 * t30194 + 0.10389515463408878255e3_f64 * t1014 * t7108 * t4323 * t2602;
    t30297
}
