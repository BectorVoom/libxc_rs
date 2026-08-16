//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 608/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk608(t8282: f64, t959: f64, t1555: f64, t26: f64, t1557: f64, t469: f64, t356: f64, t1570: f64, t1800: f64, t942: f64, t100: f64, t1587: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11720 = t8282 * t959;
    let t11755 = t26 * t1555;
    let t11756 = t469 * t1557;
    let t11761 = t26 * t356;
    let t11762 = t469 * t1570;
    let t11766 = t1800 * t942;
    let t11810 = t1587 * t100;
    (t11720, t11755, t11756, t11761, t11762, t11766, t11810)
}
