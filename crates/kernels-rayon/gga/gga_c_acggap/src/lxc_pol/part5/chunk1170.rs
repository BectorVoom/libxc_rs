//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1170/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1170(t1165: f64, t1180: f64, t12995: f64, t12999: f64, t1552: f64, t16174: f64, t16185: f64, t16191: f64, t16203: f64, t16205: f64, t16207: f64, t16319: f64, t21128: f64, t21136: f64, t21141: f64, t21146: f64) -> f64 {
    let t21157 = 0.85748036236139473944e-3_f64 * t1180 * t1165 * t1552 * t21128 - 0.34299214494455789578e-2_f64 * t21136 - 0.17149607247227894789e-2_f64 * t21141 - 0.34299214494455789578e-1_f64 * t21146 + 0.25724410870841842183e-2_f64 * t16174 + 0.68598428988911579156e-2_f64 * t12995 - 0.10289764348336736874e-1_f64 * t12999 + 7.0_f64 / 12.0_f64 * t16185 - 0.17149607247227894789e-2_f64 * t16191 + 0.85748036236139473944e-3_f64 * t16203 - 0.17149607247227894789e-2_f64 * t16205 - 0.85748036236139473944e-3_f64 * t16207 + 7.0_f64 / 144.0_f64 * t16319;
    t21157
}
