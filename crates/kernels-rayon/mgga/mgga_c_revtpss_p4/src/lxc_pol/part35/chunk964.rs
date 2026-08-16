//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 964/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk964(t23598: f64, t373: f64, t371: f64, t372: f64, t1651: f64, t6244: f64, t1011: f64, t1025: f64, t11859: f64, t11875: f64, t11941: f64, t15671: f64, t15926: f64, t16220: f64, t1665: f64, t19773: f64, t20005: f64, t20017: f64, t20021: f64, t20025: f64, t20030: f64, t20034: f64, t20051: f64, t20055: f64, t23994: f64, t23999: f64, t24009: f64, t24013: f64, t24017: f64, t3115: f64, t4858: f64, t6273: f64, t6278: f64, t6339: f64) -> (f64, f64) {
    let t24022 = t373 * t23598;
    let t24024 = t371 * t372 * t24022;
    let t24031 = t6244 * t1651;
    let t24032 = t373 * t24031;
    let t24034 = t371 * t372 * t24032;
    let t24040 = 0.57165357490759649295e-3_f64 * t20005 - 0.12862205435420921092e-2_f64 * t15926 * t6273 - 0.64311027177104605458e-3_f64 * t3115 * t23994 - 0.64311027177104605458e-3_f64 * t3115 * t23999 + 0.85748036236139473944e-3_f64 * t20017 - 0.42874018118069736972e-3_f64 * t20021 - 0.85748036236139473944e-3_f64 * t20025 + 0.85748036236139473944e-3_f64 * t20030 + 0.85748036236139473944e-3_f64 * t20034 - 0.12862205435420921092e-2_f64 * t11859 * t24009 + 0.64311027177104605458e-3_f64 * t11875 * t24013 + t1011 * t24017 / 48.0_f64 - 0.64311027177104605458e-3_f64 * t4858 * t6278 - 0.21437009059034868486e-3_f64 * t1025 * t24024 - 0.64311027177104605458e-3_f64 * t19773 * t1665 + 0.12862205435420921092e-2_f64 * t15671 * t6339 - 0.12862205435420921092e-2_f64 * t11941 * t24034 + 0.47637797908966374413e-3_f64 * t20051 + 0.28582678745379824648e-3_f64 * t20055 - t16220 / 432.0_f64;
    (t24031, t24040)
}
