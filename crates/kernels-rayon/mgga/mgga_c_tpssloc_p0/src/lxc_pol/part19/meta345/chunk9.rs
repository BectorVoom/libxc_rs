//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1243/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1243(t12935: f64, t193: f64, t202: f64, t2522: f64, t2553: f64, t39585: f64, t39590: f64, t39593: f64, t40848: f64, t40887: f64, t41252: f64, t41254: f64, t41256: f64, t41258: f64, t41260: f64, t41262: f64, t41266: f64, t41580: f64, t766: f64, t870: f64, t9470: f64) -> f64 {
    let t41591 = -t39585 + t39590 + 3.0_f64 * t193 * t766 * t40848 + t193 * t202 * (t40887 + t41580) * t870 + t41252 - t39593 - 18.0_f64 * t2522 * t9470 * t2553 + 36.0_f64 * t193 * t12935 * t2553 + t41254 - t41256 - t41258 - t41260 - t41262 - t41266;
    t41591
}
