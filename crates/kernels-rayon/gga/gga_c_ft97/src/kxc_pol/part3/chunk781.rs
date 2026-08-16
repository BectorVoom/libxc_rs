//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 781/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk781(t11811: f64, t3214: f64, t11810: f64, t16083: f64, t16086: f64, t16090: f64, t16095: f64, t16099: f64, t16103: f64, t16107: f64, t16112: f64, t16117: f64, t16122: f64, t16126: f64, t16129: f64, t16133: f64, t16137: f64, t1901: f64, t446: f64) -> f64 {
    let t16140 = t11811 * t3214;
    let t16141 = t11810 * t16140;
    let t16144 = -2.0_f64 / 27.0_f64 * t16083 - 2.0_f64 / 3.0_f64 * t446 * t16086 - 2.0_f64 / 3.0_f64 * t446 * t16090 + 2.0_f64 / 3.0_f64 * t446 * t16095 - t446 * t16099 / 9.0_f64 - t446 * t16103 / 9.0_f64 - 2.0_f64 / 27.0_f64 * t446 * t16107 - 2.0_f64 * t446 * t16112 - 2.0_f64 / 3.0_f64 * t446 * t16117 + 4.0_f64 / 3.0_f64 * t446 * t16122 - t16126 / 9.0_f64 - t446 * t16129 / 3.0_f64 - t446 * t16133 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t16137 - 4.0_f64 / 3.0_f64 * t1901 * t16141;
    t16144
}
