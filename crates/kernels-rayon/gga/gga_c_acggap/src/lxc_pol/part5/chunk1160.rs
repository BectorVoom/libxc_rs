//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1160/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1160(t1165: f64, t1173: f64, t1180: f64, t15906: f64, t15914: f64, t15916: f64, t15918: f64, t15920: f64, t15922: f64, t20897: f64, t20904: f64, t20906: f64, t3403: f64, t407: f64, t4289: f64, t5735: f64, t5862: f64, t6258: f64, t930: f64) -> f64 {
    let t20924 = 0.90702367218671976884e-1_f64 * t20897 + 0.12004725073059526352e-1_f64 * t15906 + 0.34299214494455789578e-2_f64 * t1173 * t1165 * t4289 * t6258 - 0.42874018118069736972e-3_f64 * t20904 - 0.42874018118069736972e-3_f64 * t1180 * t1165 * t20906 * t407 - 0.21437009059034868486e-3_f64 * t1180 * t1165 * t5862 * t930 + 0.80031500487063509016e-2_f64 * t15914 + 0.32012600194825403606e-1_f64 * t15916 - 0.17149607247227894789e-1_f64 * t3403 * t1165 * t4289 * t5735 + 0.80031500487063509016e-1_f64 * t15918 - 0.32012600194825403606e-1_f64 * t15920 + 0.32012600194825403606e-1_f64 * t15922;
    t20924
}
