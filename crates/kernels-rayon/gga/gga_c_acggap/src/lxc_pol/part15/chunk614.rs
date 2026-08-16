//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 614/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk614(t1841: f64, t952: f64, t1846: f64, t935: f64, t1180: f64, t127: f64, t3246: f64, t3312: f64, t3314: f64, t418: f64, t4492: f64, t4494: f64, t4505: f64, t5787: f64, t5790: f64, t5792: f64, t5796: f64, t5801: f64, t5804: f64, t5807: f64, t5811: f64, t5816: f64, t5821: f64, t5827: f64, t5829: f64) -> f64 {
    let t5831 = t952 * t1841;
    let t5833 = t935 * t1846;
    let t5837 = -t3246 + t127 * t5787 / 96.0_f64 - t4492 - t4494 - 0.85748036236139473944e-3_f64 * t5790 - 0.85748036236139473944e-3_f64 * t1180 * t5792 - 0.85748036236139473944e-3_f64 * t1180 * t5796 - 0.85748036236139473944e-3_f64 * t1180 * t5801 + 0.85748036236139473944e-3_f64 * t5804 + 0.85748036236139473944e-3_f64 * t1180 * t5807 - t4505 - 0.17149607247227894789e-2_f64 * t418 * t5811 - 0.17149607247227894789e-2_f64 * t418 * t5816 - 0.17149607247227894789e-2_f64 * t418 * t5821 - 0.42874018118069736972e-2_f64 * t5827 - 0.21437009059034868486e-3_f64 * t5829 + 0.10003937560882938627e-2_f64 * t5831 - 0.21437009059034868486e-3_f64 * t5833 + 0.17149607247227894789e-2_f64 * t3312 - 0.85748036236139473944e-3_f64 * t3314;
    t5837
}
