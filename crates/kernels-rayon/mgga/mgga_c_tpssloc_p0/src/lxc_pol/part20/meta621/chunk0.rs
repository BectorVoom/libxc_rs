//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2236/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2236(t40817: f64, t157: f64, t41279: f64, t4196: f64, t4205: f64, t9868: f64, t13130: f64, t2427: f64, t41251: f64, t10121: f64, t13191: f64, t1877: f64, t2523: f64, t39563: f64, t39585: f64, t39590: f64, t39593: f64, t4307: f64, t4314: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46331 = 0.51947577317044391277e2_f64 * t40817;
    let t46334 = 36.0_f64 * t41279 * t157 * t4196;
    let t46335 = t4205 * t9868;
    let t46336 = 12.0_f64 * t46335;
    let t46338 = 12.0_f64 * t2427 * t13130;
    let t46339 = 12.0_f64 * t41251;
    let t46340 = -t10121 * t1877 * t4307 + 36.0_f64 * t13191 * t2523 * t4314 + t39563 - t39585 + t39590 - t39593 - t46331 + t46334 + t46336 + t46338 + t46339;
    (t46331, t46334, t46336, t46338, t46339, t46340)
}
