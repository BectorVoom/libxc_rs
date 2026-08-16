//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 619/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk619(t1180: f64, t1531: f64, t3316: f64, t418: f64, t4450: f64, t4524: f64, t4532: f64, t4538: f64, t4558: f64, t4561: f64, t4563: f64, t4565: f64, t4603: f64, t5842: f64, t5844: f64, t5846: f64, t5848: f64, t5850: f64, t5855: f64, t5859: f64, t5864: f64, t5869: f64, t5873: f64, t5878: f64) -> f64 {
    let t5882 = 0.85748036236139473944e-3_f64 * t3316 + 0.85748036236139473944e-3_f64 * t4524 - t4532 + 0.80031500487063509016e-2_f64 * t4538 + 0.17149607247227894789e-2_f64 * t4558 - t4561 + t4563 - t4565 + 7.0_f64 / 144.0_f64 * t5842 + 7.0_f64 / 144.0_f64 * t5844 + 7.0_f64 / 72.0_f64 * t5846 + 7.0_f64 / 288.0_f64 * t5848 - 0.80031500487063509015e-2_f64 * t5850 - 0.12862205435420921092e-2_f64 * t4450 * t5855 + 0.12862205435420921092e-2_f64 * t1531 * t5859 - 0.42874018118069736972e-3_f64 * t1180 * t5864 + 0.42874018118069736972e-3_f64 * t1180 * t5869 - 0.21437009059034868486e-3_f64 * t1180 * t5873 - 0.17149607247227894789e-2_f64 * t418 * t5878 + 0.85748036236139473945e-2_f64 * t4603;
    t5882
}
