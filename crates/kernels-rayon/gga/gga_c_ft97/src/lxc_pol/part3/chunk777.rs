//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 777/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk777(t16076: f64, t379: f64, t1909: f64, t11467: f64, t11593: f64, t16024: f64, t16027: f64, t16031: f64, t16036: f64, t16040: f64, t16044: f64, t16049: f64, t16054: f64, t16057: f64, t16062: f64, t16067: f64, t16070: f64, t16073: f64, t1901: f64, t446: f64) -> f64 {
    let t16077 = t16076 * t379;
    let t16078 = t1909 * t16077;
    let t16081 = 2.0_f64 / 9.0_f64 * t1901 * t16024 + 4.0_f64 / 9.0_f64 * t1901 * t16027 - 4.0_f64 / 27.0_f64 * t1901 * t16031 - 2.0_f64 / 9.0_f64 * t1901 * t16036 + 2.0_f64 / 9.0_f64 * t446 * t16040 + 2.0_f64 / 3.0_f64 * t446 * t16044 + 2.0_f64 / 3.0_f64 * t446 * t16049 + t11467 + t1901 * t16054 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t1901 * t16057 + 2.0_f64 / 9.0_f64 * t1901 * t16062 + 4.0_f64 / 9.0_f64 * t11593 * t16067 + 2.0_f64 / 9.0_f64 * t1901 * t16070 + 2.0_f64 / 9.0_f64 * t1901 * t16073 + t1901 * t16078 / 9.0_f64;
    t16081
}
