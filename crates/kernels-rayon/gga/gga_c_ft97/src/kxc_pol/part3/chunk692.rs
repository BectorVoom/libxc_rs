//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 692/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk692(t1775: f64, t3146: f64, t3131: f64, t1555: f64, t26: f64, t1557: f64, t469: f64, t356: f64, t1570: f64, t11069: f64, t11076: f64, t11416: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11734 = 2.0_f64 / 9.0_f64 * t1775 * t3146;
    let t11745 = 2.0_f64 / 9.0_f64 * t1775 * t3131;
    let t11755 = t26 * t1555;
    let t11756 = t469 * t1557;
    let t11761 = t26 * t356;
    let t11762 = t469 * t1570;
    let t11778 = 2.0_f64 / 9.0_f64 * t11069;
    let t11781 = 4.0_f64 / 27.0_f64 * t11076;
    let t11798 = 4.0_f64 / 9.0_f64 * t11416;
    (t11734, t11745, t11755, t11756, t11761, t11762, t11778, t11781, t11798)
}
