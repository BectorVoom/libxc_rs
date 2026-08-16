//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 586/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk586(t160: f64, t4790: f64, t1023: f64, t1058: f64, t149: f64, t165: f64, t4650: f64, t4720: f64, t4725: f64, t4806: f64, t4810: f64, t4837: f64) -> (f64, f64) {
    let t4839 = t4790 * t160;
    let t4844 = -2.0_f64 * t1023 * t1058 - t149 * t4837 - t165 * t4650 - t165 * t4720 + 4.0_f64 * t4725 - 2.0_f64 * t4806 - 4.0_f64 * t4810 + 2.0_f64 * t4839;
    (t4839, t4844)
}
