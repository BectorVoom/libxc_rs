//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 821/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk821(t12327: f64, t12319: f64, t12322: f64, t12325: f64, t12332: f64, t12336: f64, t12340: f64, t8796: f64, t8805: f64, t9065: f64, t9068: f64, t12356: f64) -> (f64, f64) {
    let t12897 = 2.0_f64 / 27.0_f64 * t12327;
    let t12905 = -2.0_f64 / 9.0_f64 * t12319 - 2.0_f64 / 3.0_f64 * t12322 + 4.0_f64 / 9.0_f64 * t12325 - t12897 + 2.0_f64 / 9.0_f64 * t12332 - 4.0_f64 / 9.0_f64 * t12336 - 4.0_f64 / 9.0_f64 * t12340 - 2.0_f64 / 9.0_f64 * t8805 - 8.0_f64 / 27.0_f64 * t9065 + t9068 / 9.0_f64 - 8.0_f64 / 81.0_f64 * t8796;
    let t12911 = 4.0_f64 / 9.0_f64 * t12356;
    (t12905, t12911)
}
