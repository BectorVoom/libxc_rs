//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2223/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2223(t40626: f64, t4199: f64, t9919: f64, t12887: f64, t67: f64, t758: f64, t9892: f64, t13123: f64, t9882: f64, t9888: f64, t118: f64, t2375: f64, t4095: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t46120 = 3.0_f64 * t40626;
    let t46125 = t4199 * t9919;
    let t46126 = 0.35089341735807877242e1_f64 * t46125;
    let t46128 = t12887 * t67 * t758;
    let t46129 = 0.54934341918019635162e-3_f64 * t46128;
    let t46130 = t4199 * t9892;
    let t46131 = 0.51947577317044391277e2_f64 * t46130;
    let t46132 = t13123 * t9882;
    let t46133 = 0.32530743900905219526e-1_f64 * t46132;
    let t46134 = t13123 * t9888;
    let t46135 = 0.48159733137676571078e0_f64 * t46134;
    let t46137 = t4095 * t118 * t2375;
    (t46120, t46126, t46129, t46131, t46133, t46135, t46137)
}
