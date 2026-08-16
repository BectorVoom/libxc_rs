//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1076/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1076(t42290: f64, t42322: f64, t761: f64, t9974: f64, t766: f64, t2526: f64, t2568: f64, t762: f64, t9895: f64, t2492: f64, t10015: f64, t8392: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42323 = t42290 + t42322;
    let t42328 = t9974 * t761;
    let t42329 = t42328 * t766;
    let t42331 = t2526 * t2526;
    let t42332 = t2568 * t42331;
    let t42334 = t9895 * t762;
    let t42339 = t2492 * t2568;
    let t42344 = t8392 * t10015;
    (t42323, t42329, t42332, t42334, t42339, t42344)
}
