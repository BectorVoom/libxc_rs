//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 592/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk592(t27617: f64, t70: f64, t213: f64, t703: f64, t684: f64, t3751: f64, t6036: f64, t2383: f64, t695: f64, t17817: f64, t209: f64, t9: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27659 = t27617 * t70;
    let t27660 = t703 * t213;
    let t27661 = t27660 * t684;
    let t27662 = t27659 * t27661;
    let t27665 = t6036 * t3751;
    let t27669 = t2383 * t695;
    let t27670 = t17817 * t27669;
    let t27671 = t9 * t209;
    (t27661, t27662, t27665, t27669, t27670, t27671)
}
