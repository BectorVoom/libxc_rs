//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 827/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk827(t11120: f64, t22547: f64, t1299: f64, t388: f64, t51: f64, t5566: f64, t1608: f64, t35: f64, t428: f64, t3065: f64, t39: f64, t78: f64) -> (f64, f64, f64, f64, f64) {
    let t22548 = t22547 * t11120;
    let t22590 = t388 * t1299;
    let t22602 = t5566 * t51;
    let t22603 = t1608 * t22602;
    let t22604 = t35 * t428;
    let t22605 = t3065 * t22604;
    let t22686 = t78 * t39;
    (t22548, t22590, t22603, t22605, t22686)
}
