//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1140/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1140(t1165: f64, t1432: f64, t15947: f64, t3361: f64, t3375: f64, t6351: f64, t1173: f64, t1176: f64, t1180: f64, t1181: f64, t12536: f64, t15469: f64, t15479: f64, t15483: f64, t20417: f64, t20422: f64, t20430: f64, t20433: f64, t20441: f64, t4643: f64, t5270: f64, t5852: f64) -> f64 {
    let t20446 = t3361 * t1165 * t15947 * t1432;
    let t20448 = t3375 * t6351;
    let t20450 = 0.34299214494455789578e-2_f64 * t1173 * t20417 * t1176 - 0.20007875121765877254e-2_f64 * t12536 + 0.16006300097412701803e-1_f64 * t20422 + 0.68598428988911579156e-2_f64 * t15469 + 0.34299214494455789578e-2_f64 * t1173 * t1181 * t5852 * t5270 + 0.68598428988911579156e-2_f64 * t15479 - 0.16006300097412701803e-1_f64 * t20430 + 0.17149607247227894789e-2_f64 * t1180 * t1181 * t4643 * t20433 + 0.34299214494455789578e-2_f64 * t20441 - 0.48018900292238105409e-1_f64 * t15483 - 0.68598428988911579156e-2_f64 * t20446 + 0.42874018118069736972e-3_f64 * t20448;
    t20450
}
