//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 292/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk292(t1160: f64, t241: f64, t258: f64, t1089: f64, t1134: f64, t1156: f64, t764: f64) -> (f64, f64) {
    let t1162 = t241 * t1160 * t258;
    let t1168 = t1156 / 2.0_f64 - t764 - t1089 / 3.0_f64 - t1134;
    (t1162, t1168)
}
