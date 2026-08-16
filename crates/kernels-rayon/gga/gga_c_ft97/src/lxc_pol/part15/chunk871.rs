//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 871/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk871(t39673: f64, t1570: f64, t2178: f64, t1557: f64, t604: f64, t7800: f64, t605: f64, t9132: f64, t157: f64, t40465: f64, t24: f64, t32905: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t40530 = 140.0_f64 / 243.0_f64 * t39673;
    let t40599 = 280.0_f64 / 243.0_f64 * t39673;
    let t40759 = t2178 * t1570;
    let t40766 = t2178 * t1557;
    let t40771 = t604 * t7800;
    let t40792 = t9132 * t605;
    let t40808 = t40465 * t157;
    let t40830 = t24 * t32905;
    (t40530, t40599, t40759, t40766, t40771, t40792, t40808, t40830)
}
