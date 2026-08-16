//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 764/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk764(t11913: f64, t11962: f64, t11966: f64, t11969: f64, t11973: f64, t11977: f64, t11981: f64, t11984: f64, t11990: f64, t11994: f64, t1901: f64, t446: f64, t8499: f64, t8516: f64, t8523: f64, t8526: f64, t8534: f64) -> f64 {
    let t11997 = -t11913 - 2.0_f64 / 27.0_f64 * t8499 + 8.0_f64 / 27.0_f64 * t8516 + t8523 / 9.0_f64 + t8526 / 27.0_f64 - t8534 - t446 * t11962 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t11966 + 2.0_f64 / 3.0_f64 * t446 * t11969 - t446 * t11973 / 9.0_f64 - 2.0_f64 / 27.0_f64 * t446 * t11977 - t11981 - 2.0_f64 / 27.0_f64 * t1901 * t11984 - 10.0_f64 / 81.0_f64 * t1901 * t11990 + t1901 * t11994 / 9.0_f64;
    t11997
}
