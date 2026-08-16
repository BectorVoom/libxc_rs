//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 659/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk659(t157: f64, t9132: f64, t2101: f64, t605: f64, t9071: f64, t151: f64, t3051: f64, t1771: f64, t588: f64, t2: f64, t9114: f64, t583: f64, t8282: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9133 = t9132 * t157;
    let t9144 = t2101 * t605;
    let t9166 = 28.0_f64 / 27.0_f64 * t9071;
    let t9178 = 28.0_f64 / 27.0_f64 * t3051 * t151;
    let t9179 = t1771 * t588;
    let t9192 = t9114 * t2;
    let t9202 = t8282 * t583;
    (t9133, t9144, t9166, t9178, t9179, t9192, t9202)
}
