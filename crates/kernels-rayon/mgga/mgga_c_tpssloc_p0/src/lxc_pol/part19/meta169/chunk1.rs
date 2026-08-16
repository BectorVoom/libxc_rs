//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 800/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk800(t241: f64, t6589: f64, t67: f64, t820: f64, t9458: f64, t2613: f64, t68: f64) -> (f64, f64, f64) {
    let t9607 = t241 * t6589 * t67;
    let t9609 = t9607 * t820 * t9458;
    let t9612 = t2613 * t68;
    (t9607, t9609, t9612)
}
