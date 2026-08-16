//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1238/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1238(t13258: f64, t9634: f64, t9629: f64, t2379: f64, t2632: f64, t776: f64, t9975: f64, t6589: f64, t67: f64, t246: f64, t232: f64, t9458: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41435 = t13258 * t9634;
    let t41437 = t13258 * t9629;
    let t41448 = t2632 * t2379;
    let t41453 = t9975 * t776;
    let t41466 = t6589 * t67;
    let t41467 = t41466 * t246;
    let t41468 = t232 * t9458;
    (t41435, t41437, t41448, t41453, t41467, t41468)
}
