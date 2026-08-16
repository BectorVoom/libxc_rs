//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 333/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk333(t1675: f64, t1683: f64, t41: f64, t42: f64, t78: f64, t388: f64, t4: f64, t40: f64, t39: f64, t53: f64, t11: f64, t55: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1685 = 0.44057546758024691357e0_f64 * t41 * t42 * t1675 + 0.18770038718167957794e-1_f64 * t1683;
    let t1686 = t78 * t1685;
    let t1687 = t388 * t1686;
    let t1689 = t40 * t4;
    let t1690 = t39 * t1689;
    let t1691 = t53 * t53;
    let t1692 = t11 * t1691;
    let t1693 = t55 * t55;
    (t1685, t1687, t1689, t1690, t1691, t1692, t1693)
}
