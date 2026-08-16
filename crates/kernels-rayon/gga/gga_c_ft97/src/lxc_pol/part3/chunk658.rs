//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 658/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk658(t9071: f64, t1984: f64, t378: f64, t582: f64, t597: f64, t1554: f64, t525: f64, t157: f64, t1557: f64, t604: f64, t1570: f64, t355: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9072 = 14.0_f64 / 81.0_f64 * t9071;
    let t9073 = t378 * t1984;
    let t9099 = t582 * t597;
    let t9114 = t1554 * t525;
    let t9115 = t9114 * t157;
    let t9121 = t604 * t1557;
    let t9127 = t604 * t1570;
    let t9132 = t355 * t1984;
    (t9072, t9073, t9099, t9114, t9115, t9121, t9127, t9132)
}
