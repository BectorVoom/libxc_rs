//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2361/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2361(t12899: f64, t16662: f64, t1877: f64, t20753: f64, t20769: f64, t20778: f64, t39658: f64, t40772: f64, t4314: f64, t4315: f64, t46341: f64, t46438: f64, t5544: f64, t67495: f64, t67496: f64, t67497: f64, t67498: f64, t868: f64) -> f64 {
    let t68407 = -6.0_f64 * t1877 * t20778 * t40772 * t868 + 18.0_f64 * t12899 * t4314 * t5544 + 18.0_f64 * t16662 * t4314 * t4315 + 18.0_f64 * t20753 * t46341 + 18.0_f64 * t20769 * t46341 - t39658 + t46438 + t67495 + t67496 + t67497 + t67498;
    t68407
}
