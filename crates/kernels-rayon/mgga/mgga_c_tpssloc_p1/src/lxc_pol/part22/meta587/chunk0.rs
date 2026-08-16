//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2098/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2098(t46206: f64, t4199: f64, t9494: f64, t12945: f64, t2427: f64, t12858: f64, t2528: f64, t2371: f64, t13123: f64, t9885: f64, t1409: f64, t2516: f64, t4194: f64, t607: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t46207 = 12.0_f64 * t46206;
    let t46208 = t4199 * t9494;
    let t46217 = t2427 * t12945;
    let t46218 = 12.0_f64 * t46217;
    let t46234 = t12858 * t2528;
    let t46235 = 0.51947577317044391276e2_f64 * t46234;
    let t46236 = t12858 * t2371;
    let t46237 = 0.35089341735807877242e1_f64 * t46236;
    let t46278 = t13123 * t9885;
    let t46291 = t4194 * t2516 * t1409 * t607;
    (t46207, t46208, t46218, t46235, t46237, t46278, t46291)
}
