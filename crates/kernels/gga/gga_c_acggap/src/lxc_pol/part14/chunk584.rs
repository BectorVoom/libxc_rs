//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 584/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk584<F: Float>(t1181: F, t1182: F, t5862: F, t1838: F, t435: F, t1165: F, t1188: F, t407: F, t1772: F, t301: F, t1089: F, t368: F, t1180: F, t1531: F, t3316: F, t418: F, t4450: F, t4524: F, t4532: F, t4538: F, t4558: F, t4561: F, t4563: F, t4565: F, t4603: F, t5842: F, t5844: F, t5846: F, t5848: F, t5850: F, t5855: F, t5859: F) -> (F, F, F, F, F, F) {
    let t5864 = t1181 * t5862 * t1182;
    let t5867 = t435 * t1838;
    let t5869 = t1165 * t5867 * t1188;
    let t5873 = t1165 * t5862 * t407;
    let t5876 = t1772 * t301;
    let t5878 = t1089 * t368 * t5876;
    let t5882 = 0.85748036236139473944e-3 * t3316 + 0.85748036236139473944e-3 * t4524 - t4532 + 0.80031500487063509016e-2 * t4538 + 0.17149607247227894789e-2 * t4558 - t4561 + t4563 - t4565 + 7.0 / 144.0 * t5842 + 7.0 / 144.0 * t5844 + 7.0 / 72.0 * t5846 + 7.0 / 288.0 * t5848 - 0.80031500487063509015e-2 * t5850 - 0.12862205435420921092e-2 * t4450 * t5855 + 0.12862205435420921092e-2 * t1531 * t5859 - 0.42874018118069736972e-3 * t1180 * t5864 + 0.42874018118069736972e-3 * t1180 * t5869 - 0.21437009059034868486e-3 * t1180 * t5873 - 0.17149607247227894789e-2 * t418 * t5878 + 0.85748036236139473945e-2 * t4603;
    (t5864, t5869, t5873, t5876, t5878, t5882)
}
