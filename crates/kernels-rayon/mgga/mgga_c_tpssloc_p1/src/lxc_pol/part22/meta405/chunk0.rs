//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1704/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1704(t11219: f64, t18206: f64, t136: f64, t18211: f64, t3297: f64, t18215: f64, t6014: f64, t699: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18496 = t11219 * t18206;
    let t18497 = t136 * t18496;
    let t18499 = t3297 * t18211;
    let t18500 = t136 * t18499;
    let t18502 = t3297 * t18215;
    let t18503 = t136 * t18502;
    let t18505 = t699 * t6014;
    (t18496, t18497, t18499, t18500, t18502, t18503, t18505)
}
