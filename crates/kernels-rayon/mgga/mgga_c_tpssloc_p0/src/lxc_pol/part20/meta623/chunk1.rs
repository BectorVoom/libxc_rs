//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2244/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2244(t13115: f64, t9932: f64, t32: f64, t4094: f64, t2659: f64, t1530: f64, t193: f64, t39658: f64, t46426: f64, t46432: f64, t46434: f64, t46436: f64, t46438: f64, t46439: f64, t46444: f64, t766: f64, t870: f64, t9458: f64) -> (f64, f64, f64) {
    let t46445 = t13115 * t9932;
    let t46446 = 36.0_f64 * t46445;
    let t46447 = t32 * t4094;
    let t46449 = 36.0_f64 * t46447 * t2659;
    let t46450 = 6.0_f64 * t1530 * t193 * t870 * t9458 + 3.0_f64 * t193 * t46426 * t766 - t39658 + t46432 - t46434 + t46436 + t46438 + t46439 + t46444 + t46446 + t46449;
    (t46446, t46449, t46450)
}
