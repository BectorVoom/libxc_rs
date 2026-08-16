//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 644/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk644(t1609: f64, t77: f64, t1593: f64, t1608: f64, t1615: f64, t1630: f64, t1711: f64, t371: f64, t407: f64, t391: f64, t625: f64, t68: f64, t72: f64) -> (f64, f64, f64, f64, f64) {
    let t8007 = t77 * t1609;
    let t8008 = t8007 * t1593;
    let t8009 = t1608 * t8008;
    let t8014 = t1615 * t1630;
    let t8015 = t1608 * t8014;
    let t8042 = t371 * t1711;
    let t8050 = t407 * t407;
    let t8051 = 1.0_f64 / t8050;
    let t8074 = t68 * t391 * t625 * t72;
    (t8009, t8015, t8042, t8051, t8074)
}
