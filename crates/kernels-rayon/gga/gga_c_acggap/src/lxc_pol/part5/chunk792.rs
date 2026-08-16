//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 792/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk792(t1579: f64, t336: f64, t513: f64, t1524: f64, t535: f64, t1755: f64, t3409: f64, t1173: f64, t1180: f64, t335: f64, t3396: f64, t3571: f64, t3574: f64, t3649: f64, t367: f64, t4901: f64, t4906: f64, t4910: f64, t4918: f64, t4926: f64, t4928: f64, t6140: f64, t6145: f64, t6148: f64, t6153: f64, t6158: f64, t6161: f64, t6164: f64, t6167: f64) -> (f64, f64, f64) {
    let t6171 = t336 * t1579 * t513;
    let t6175 = t336 * t535 * t1524;
    let t6180 = t3409 * t1755;
    let t6182 = 0.80031500487063509015e-2_f64 * t4901 - t4906 + t4910 + t4918 - 0.85748036236139473944e-3_f64 * t4926 - 0.40015750243531754508e-2_f64 * t4928 - 0.10289764348336736873e-1_f64 * t3396 * t6140 + 0.17149607247227894789e-2_f64 * t6145 - 0.21437009059034868486e-3_f64 * t1180 * t6148 + 0.85748036236139473944e-3_f64 * t1180 * t6153 + 0.21437009059034868486e-3_f64 * t6158 + 0.85748036236139473944e-3_f64 * t1173 * t6161 - 7.0_f64 / 144.0_f64 * t6164 - t335 * t6167 / 48.0_f64 - t367 * t6171 / 48.0_f64 - t367 * t6175 / 48.0_f64 - 35.0_f64 / 216.0_f64 * t3571 - 35.0_f64 / 432.0_f64 * t3574 + 0.40015750243531754507e-2_f64 * t6180 + t3649;
    (t6171, t6175, t6182)
}
