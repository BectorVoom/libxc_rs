//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 622/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk622(t1449: f64, t4267: f64, t1181: f64, t3539: f64, t5852: f64, t1165: f64, t3463: f64, t1150: f64, t1173: f64, t1180: f64, t1531: f64, t335: f64, t3358: f64, t3373: f64, t3376: f64, t3396: f64, t3428: f64, t3462: f64, t367: f64, t4463: f64, t4627: f64, t5884: f64, t5891: f64, t5895: f64, t5899: f64, t5903: f64, t5907: f64, t5910: f64, t5913: f64, t5916: f64, t5919: f64, t5924: f64, t5928: f64) -> (f64, f64, f64, f64) {
    let t5931 = t4267 * t1449;
    let t5932 = t1181 * t5931;
    let t5936 = t1181 * t5852 * t3539;
    let t5940 = t1165 * t5852 * t3463;
    let t5943 = -7.0_f64 / 288.0_f64 * t5884 - 0.17149607247227894789e-2_f64 * t3358 - 0.20007875121765877254e-2_f64 * t3373 + 0.42874018118069736972e-3_f64 * t3376 + 0.21437009059034868486e-3_f64 * t3428 + 0.85748036236139473944e-3_f64 * t1173 * t5891 - 0.42874018118069736972e-3_f64 * t1180 * t5895 + 0.42874018118069736972e-3_f64 * t1180 * t5899 + 0.68598428988911579156e-2_f64 * t3396 * t5903 - t4627 + t335 * t5907 / 48.0_f64 + t335 * t5910 / 24.0_f64 + t367 * t5913 / 24.0_f64 + t1150 * t5916 / 8.0_f64 + t335 * t5919 / 24.0_f64 - 0.85748036236139473944e-3_f64 * t1531 * t5924 + 0.17149607247227894789e-1_f64 * t4463 * t5928 + 0.68598428988911579156e-2_f64 * t3396 * t5932 + 0.85748036236139473944e-3_f64 * t1531 * t5936 - 0.17149607247227894789e-2_f64 * t3462 * t5940;
    (t5932, t5936, t5940, t5943)
}
