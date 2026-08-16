//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 913/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk913(t4811: f64, t8232: f64, t4730: f64, t1637: f64, t4792: f64, t89: f64, t4815: f64, t4735: f64, t49266: f64, t49337: f64, t1526: f64, t38308: f64, t4641: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t63795 = t8232 * t4811;
    let t64001 = t8232 * t4730;
    let t64231 = t89 * t1637 * t4792;
    let t64255 = t8232 * t4815;
    let t64279 = t8232 * t4735;
    let t64491 = 56.0_f64 / 81.0_f64 * t49266;
    let t64516 = 56.0_f64 / 243.0_f64 * t49337;
    let t64663 = t1526 * t38308 * t4641;
    (t63795, t64001, t64231, t64255, t64279, t64491, t64516, t64663)
}
