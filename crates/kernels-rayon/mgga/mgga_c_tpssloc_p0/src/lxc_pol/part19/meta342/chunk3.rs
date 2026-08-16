//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1222/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1222(t2588: f64, t40341: f64, t12998: f64, t2553: f64, t686: f64, t9524: f64, t13012: f64, t9566: f64, t207: f64, t215: f64, t39933: f64, t40344: f64, t795: f64) -> (f64, f64, f64, f64, f64) {
    let t41200 = 0.99537037037037037035e-1_f64 * t40341 * t2588;
    let t41203 = t12998 * t686 * t9524 * t2553;
    let t41205 = t13012 * t9566;
    let t41209 = 0.14979423868312757201e0_f64 * t39933 * t207 * t215;
    let t41212 = 0.11265432098765432099e0_f64 * t40344 * t207 * t795;
    (t41200, t41203, t41205, t41209, t41212)
}
