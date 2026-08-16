//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1225/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1225(t20432: f64, t944: f64, t3379: f64, t5712: f64, t3375: f64, t6157: f64, t1163: f64, t1165: f64, t4162: f64, t5852: f64, t1180: f64, t1181: f64, t1531: f64, t1552: f64, t157: f64, t17216: f64, t20972: f64, t22383: f64, t22388: f64, t22397: f64, t22399: f64, t3462: f64, t4267: f64, t4643: f64, t4838: f64) -> (f64, f64) {
    let t22401 = t20432 * t944;
    let t22410 = t3379 * t5712;
    let t22417 = t3375 * t6157;
    let t22421 = t1163 * t1165 * t5852 * t4162;
    let t22424 = 35.0_f64 / 432.0_f64 * t22383 - 0.85748036236139473944e-3_f64 * t22388 + 0.17149607247227894789e-2_f64 * t1180 * t1165 * t1552 * t20972 - 0.17149607247227894789e-2_f64 * t22397 + 0.80031500487063509014e-2_f64 * t22399 + 0.68598428988911579156e-2_f64 * t3462 * t1165 * t4267 * t22401 - 0.34299214494455789578e-2_f64 * t1531 * t1181 * t4643 * t22401 + 0.68598428988911579156e-2_f64 * t22410 + 0.85748036236139473944e-3_f64 * t1180 * t1165 * t1552 * t157 * t4838 + 0.42874018118069736972e-3_f64 * t22417 + 0.21437009059034868486e-3_f64 * t22421 - 35.0_f64 / 108.0_f64 * t17216;
    (t22401, t22424)
}
