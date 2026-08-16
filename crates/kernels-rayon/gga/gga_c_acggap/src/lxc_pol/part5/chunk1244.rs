//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1244/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1244(t1089: f64, t1101: f64, t1743: f64, t384: f64, t1096: f64, t1165: f64, t13664: f64, t17484: f64, t17501: f64, t17503: f64, t17505: f64, t17507: f64, t17509: f64, t17511: f64, t17513: f64, t17521: f64, t20764: f64, t3396: f64) -> f64 {
    let t22843 = t384 * t1089 * t1743 * t1101;
    let t22845 = 0.85748036236139473944e-3_f64 * t13664 - 0.16006300097412701803e-1_f64 * t17484 - 0.10289764348336736874e-1_f64 * t3396 * t1165 * t20764 * t1096 - 0.85748036236139473944e-3_f64 * t17501 + 0.16006300097412701803e-1_f64 * t17503 - 0.16006300097412701803e-1_f64 * t17505 - 0.12004725073059526352e-1_f64 * t17507 + 0.80031500487063509015e-2_f64 * t17509 - 0.24009450146119052704e-1_f64 * t17511 - 0.17149607247227894789e-1_f64 * t17513 + 0.80031500487063509015e-1_f64 * t17521 + 0.34299214494455789578e-2_f64 * t22843;
    t22845
}
