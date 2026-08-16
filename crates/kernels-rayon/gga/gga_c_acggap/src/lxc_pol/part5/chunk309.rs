//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 309/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk309(t1072: f64, t19: f64, t661: f64, t1068: f64, t136: f64, t1048: f64, t1050: f64, t1054: f64, t1057: f64, t1063: f64, t1066: f64) -> (f64, f64, f64, f64) {
    let t1074 = t1072 * t19 * t661;
    let t1075 = t1068 * t136 * t1074;
    let t1076 = t1075 / 12.0_f64;
    let t1077 = t1048 + 2.0_f64 / 3.0_f64 * t1050 - t1054 + t1057 / 2.0_f64 - t1063 / 12.0_f64 - t1066 / 4.0_f64 + t1076;
    (t1074, t1075, t1076, t1077)
}
