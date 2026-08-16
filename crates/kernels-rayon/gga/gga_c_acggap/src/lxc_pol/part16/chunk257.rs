//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 257/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk257(t1072: f64, t19: f64, t661: f64, t1068: f64, t136: f64, t141: f64, t435: f64) -> (f64, f64, f64, f64) {
    let t1074 = t1072 * t19 * t661;
    let t1075 = t1068 * t136 * t1074;
    let t1076 = t1075 / 12.0_f64;
    let t1083 = t141 * t435;
    (t1074, t1075, t1076, t1083)
}
