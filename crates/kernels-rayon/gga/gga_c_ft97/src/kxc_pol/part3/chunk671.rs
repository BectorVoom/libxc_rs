//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 671/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk671(t9698: f64, t259: f64, t89: f64, t9555: f64, t2399: f64, t756: f64, t2567: f64, t754: f64, t2492: f64, t762: f64, t241: f64, t9568: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9972 = 28.0_f64 / 81.0_f64 * t9698;
    let t9982 = 28.0_f64 / 81.0_f64 * t89 * t9555 * t259;
    let t10000 = t89 * t2399 * t756;
    let t10002 = t754 * t2567;
    let t10007 = t2492 * t762;
    let t10024 = t9568 * t241;
    (t9972, t9982, t10000, t10002, t10007, t10024)
}
