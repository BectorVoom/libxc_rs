//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 416/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk416(t2718: f64, t2719: f64, t252: f64, t2627: f64, t2633: f64, t814: f64, t852: f64, t829: f64, t2679: f64, t860: f64, t2684: f64, t235: f64, t2710: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2720 = t2718 * t2719;
    let t2728 = t2627 * t252;
    let t2729 = t2728 * t2633;
    let t2732 = t814 * t852;
    let t2733 = t2732 * t829;
    let t2736 = t860 * t2679;
    let t2738 = t860 * t2684;
    let t2740 = t235 * t2710;
    (t2720, t2729, t2733, t2736, t2738, t2740)
}
