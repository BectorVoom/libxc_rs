//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 933/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk933(t1526: f64, t42262: f64, t5198: f64, t1882: f64, t20146: f64, t1546: f64, t20149: f64, t89: f64, t20134: f64, t7780: f64, t20157: f64, t20104: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t72992 = t1526 * t42262 * t5198;
    let t73256 = t1882 * t20146;
    let t73259 = t89 * t1546 * t20149;
    let t73262 = t89 * t7780 * t20134;
    let t73276 = t89 * t1546 * t20157;
    let t73299 = t1882 * t20104;
    (t72992, t73256, t73259, t73262, t73276, t73299)
}
