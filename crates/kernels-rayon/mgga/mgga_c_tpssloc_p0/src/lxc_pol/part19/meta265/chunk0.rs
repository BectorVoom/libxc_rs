//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1016/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1016(t248: f64, t3252: f64, t3521: f64, t1227: f64, t3248: f64, t11172: f64, t1230: f64, t11163: f64, t1009: f64, t3481: f64, t1011: f64, t1212: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11797 = t248 * t3521 * t3252;
    let t11798 = t1227 * t11797;
    let t11801 = t248 * t3521 * t3248;
    let t11802 = t1227 * t11801;
    let t11805 = t248 * t1230 * t11172;
    let t11809 = t248 * t1230 * t11163;
    let t11812 = t3481 * t1009;
    let t11813 = t11812 * t1011;
    let t11814 = t11813 * t1212;
    (t11797, t11798, t11801, t11802, t11805, t11809, t11812, t11814)
}
