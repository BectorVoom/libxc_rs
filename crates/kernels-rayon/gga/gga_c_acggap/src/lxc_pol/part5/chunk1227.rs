//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1227/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1227(t3431: f64, t5608: f64, t13505: f64, t13507: f64, t13509: f64, t13517: f64, t13521: f64, t13532: f64, t13539: f64, t17254: f64, t17258: f64, t17262: f64, t17266: f64) -> f64 {
    let t22455 = t3431 * t5608;
    let t22459 = -0.12004725073059526352e-1_f64 * t13505 + 0.12004725073059526352e-1_f64 * t13507 - 0.85748036236139473944e-3_f64 * t13509 + 0.40015750243531754508e-2_f64 * t13517 - 0.42874018118069736972e-3_f64 * t13521 - 0.68598428988911579156e-2_f64 * t17254 - 0.34299214494455789578e-2_f64 * t17258 + 0.17149607247227894789e-2_f64 * t17262 - 0.17149607247227894789e-2_f64 * t17266 + 0.32012600194825403606e-1_f64 * t22455 - 0.4801890029223810541e-1_f64 * t13532 + 0.85748036236139473944e-3_f64 * t13539;
    t22459
}
