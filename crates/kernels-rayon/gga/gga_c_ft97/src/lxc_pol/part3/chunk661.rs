//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 661/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk661(t2178: f64, t597: f64, t571: f64, t8232: f64, t1637: f64, t599: f64, t89: f64, t143: f64, t7954: f64, t9065: f64, t8796: f64, t9071: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9276 = t597 * t2178;
    let t9298 = t8232 * t571;
    let t9321 = t89 * t1637 * t599;
    let t9327 = t7954 * t143;
    let t9369 = 4.0_f64 / 9.0_f64 * t9065;
    let t9371 = 4.0_f64 / 27.0_f64 * t8796;
    let t9383 = 28.0_f64 / 81.0_f64 * t9071;
    (t9276, t9298, t9321, t9327, t9369, t9371, t9383)
}
