//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 319/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk319(t35: f64, t63: f64, t78: f64, t37: f64, t62: f64, t371: f64, t66: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1597 = t35 * t35;
    let t1598 = t1597 * t63;
    let t1599 = t1598 * t78;
    let t1602 = t37 * t62;
    let t1603 = t1602 * t78;
    let t1608 = t371 * t66;
    let t1609 = f64::powf(4.0_f64, 1.0_f64 / 15.0_f64);
    (t1597, t1599, t1602, t1603, t1608, t1609)
}
