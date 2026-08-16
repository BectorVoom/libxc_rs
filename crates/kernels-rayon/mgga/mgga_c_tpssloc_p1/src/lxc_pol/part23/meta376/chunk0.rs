//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1177/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1177(t4211: f64, t9874: f64, t1472: f64, t9862: f64, t1519: f64, t9971: f64, t1496: f64, t41083: f64, t1516: f64, t40965: f64, t4166: f64, t9637: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46433 = t4211 * t9874;
    let t46439 = t1472 * t9862;
    let t46524 = t9971 * t1519;
    let t46546 = t41083 * t1496;
    let t46577 = t40965 * t1516;
    let t46657 = t4166 * t9637;
    (t46433, t46439, t46524, t46546, t46577, t46657)
}
