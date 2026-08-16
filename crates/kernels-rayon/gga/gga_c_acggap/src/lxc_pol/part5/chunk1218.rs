//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1218/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1218(t13092: f64, t5903: f64, t1084: f64, t1165: f64, t1180: f64, t1181: f64, t1531: f64, t17111: f64, t17113: f64, t17116: f64, t17118: f64, t17120: f64, t17128: f64, t1899: f64, t20400: f64, t3396: f64, t4199: f64, t4450: f64, t4463: f64, t4643: f64, t5265: f64, t5862: f64, t945: f64, t955: f64) -> f64 {
    let t22253 = t13092 * t5903;
    let t22272 = -0.16006300097412701803e-1_f64 * t17111 + 0.80031500487063509016e-2_f64 * t17113 - 0.16006300097412701803e-1_f64 * t17116 - 0.85748036236139473945e-2_f64 * t17118 + 0.17149607247227894789e-2_f64 * t17120 + 0.42874018118069736972e-3_f64 * t1180 * t1165 * t1899 * t955 - 0.64025200389650807212e-1_f64 * t22253 - 0.34299214494455789578e-1_f64 * t4463 * t1181 * t4643 * t5265 - 0.12862205435420921092e-2_f64 * t4450 * t1165 * t5862 * t4199 + 0.12862205435420921092e-2_f64 * t1531 * t1165 * t5862 * t945 + 0.68598428988911579156e-2_f64 * t3396 * t1181 * t20400 * t1084 - 0.68598428988911579156e-2_f64 * t17128;
    t22272
}
