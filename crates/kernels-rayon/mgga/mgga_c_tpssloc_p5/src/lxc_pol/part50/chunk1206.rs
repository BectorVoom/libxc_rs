//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1206/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1206(t30840: f64, t4571: f64, t30829: f64, t4630: f64, t113380: f64, t25638: f64, t113381: f64, t113388: f64, t113397: f64, t113400: f64, t113454: f64, t1618: f64, t1622: f64, t25585: f64, t25601: f64, t30817: f64, t4636: f64, t8384: f64) -> f64 {
    let t119312 = t30840 * t4571;
    let t119316 = t30829 * t4630;
    let t119322 = t25638 * t113380;
    let t119324 = -0.40372756094140390856e-3_f64 * t113381 - 0.32298204875312312685e-2_f64 * t25585 * t8384 + t30840 * t4636 / 2304.0_f64 - t113454 * t1622 / 432.0_f64 + t119312 / 3456.0_f64 - t113397 * t1618 / 288.0_f64 + t119316 / 2304.0_f64 + t113388 / 2304.0_f64 + t113400 / 3456.0_f64 - 0.40372756094140390856e-3_f64 * t25601 * t30817 - 0.40372756094140390856e-3_f64 * t119322;
    t119324
}
