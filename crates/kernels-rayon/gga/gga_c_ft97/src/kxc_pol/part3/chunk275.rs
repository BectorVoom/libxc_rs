//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 275/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk275(t1039: f64, t579: f64, t91: f64, t1000: f64, t1020: f64, t594: f64) -> (f64, f64) {
    let t1041 = t91 * t579 * t1039;
    let t1045 = t1041 / 6.0_f64 - t594 - t1000 / 9.0_f64 - t1020 / 3.0_f64;
    (t1041, t1045)
}
