//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1254/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1254(t1106: f64, t1181: f64, t1899: f64, t3391: f64, t1165: f64, t17710: f64, t17718: f64, t17721: f64, t17725: f64, t23063: f64, t23065: f64, t23068: f64, t23070: f64, t23077: f64, t23081: f64, t3396: f64, t4665: f64, t6138: f64) -> f64 {
    let t23088 = t3391 * t1181 * t1899 * t1106;
    let t23090 = 0.16006300097412701803e-1_f64 * t23063 + 0.16006300097412701803e-1_f64 * t23065 - 0.68598428988911579156e-2_f64 * t17710 - 0.32012600194825403606e-1_f64 * t23068 - 0.20579528696673473746e-1_f64 * t23070 - 0.10289764348336736873e-1_f64 * t3396 * t1165 * t6138 * t4665 + 0.34299214494455789578e-2_f64 * t23077 + 0.34299214494455789578e-2_f64 * t23081 + 0.68598428988911579156e-2_f64 * t17718 + 0.34299214494455789578e-2_f64 * t17721 + 0.34299214494455789578e-2_f64 * t17725 + 0.17149607247227894789e-2_f64 * t23088;
    t23090
}
