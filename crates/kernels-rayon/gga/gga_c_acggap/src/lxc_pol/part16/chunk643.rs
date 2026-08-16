//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 643/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk643(t1750: f64, t3379: f64, t1165: f64, t1748: f64, t4289: f64, t1298: f64, t157: f64, t1532: f64, t1410: f64, t944: f64, t1173: f64, t1180: f64, t1531: f64, t418: f64, t5007: f64, t5017: f64, t5086: f64, t5090: f64, t5092: f64, t5102: f64, t5104: f64, t5126: f64, t5131: f64, t5135: f64, t5149: f64, t6228: f64, t6237: f64, t6241: f64, t6245: f64, t6249: f64) -> (f64, f64, f64, f64, f64) {
    let t6252 = t3379 * t1750;
    let t6255 = t1165 * t4289 * t1748;
    let t6258 = t157 * t1298;
    let t6260 = t1165 * t1532 * t6258;
    let t6263 = t944 * t1410;
    let t6265 = t1165 * t1532 * t6263;
    let t6268 = -0.85748036236139473944e-3_f64 * t418 * t6228 - t5007 + 0.80031500487063509016e-2_f64 * t5017 - t5086 - 0.85748036236139473944e-3_f64 * t5090 - 0.42874018118069736972e-3_f64 * t5092 - 0.17149607247227894789e-2_f64 * t5102 - 0.80031500487063509015e-2_f64 * t5104 + 0.85748036236139473944e-3_f64 * t1173 * t6237 - 0.42874018118069736972e-3_f64 * t1180 * t6241 + 0.42874018118069736972e-3_f64 * t1180 * t6245 - 0.42874018118069736972e-3_f64 * t1180 * t6249 + 0.17149607247227894789e-2_f64 * t6252 + 0.17149607247227894789e-2_f64 * t1173 * t6255 + 0.17149607247227894789e-2_f64 * t1173 * t6260 + 0.85748036236139473944e-3_f64 * t1531 * t6265 + t5126 - t5131 + t5135 - t5149;
    (t6255, t6260, t6263, t6265, t6268)
}
