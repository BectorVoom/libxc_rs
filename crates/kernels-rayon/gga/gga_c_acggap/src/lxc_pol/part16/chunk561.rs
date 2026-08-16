//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 561/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk561(t509: f64, t987: f64, t1165: f64, t1532: f64, t4162: f64, t1163: f64, t1530: f64, t3371: f64, t1535: f64, t1162: f64, t4180: f64, t1016: f64, t513: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4369 = t987 * t509;
    let t4372 = t1165 * t1532 * t4162;
    let t4373 = t1163 * t4372;
    let t4389 = t1530 * t3371;
    let t4391 = 0.40015750243531754508e-2_f64 * t4389 * t1535;
    let t4396 = t4180 * t1162;
    let t4398 = 0.85748036236139473944e-3_f64 * t4396 * t1535;
    let t4417 = t1016 * t513;
    (t4369, t4372, t4373, t4391, t4398, t4417)
}
