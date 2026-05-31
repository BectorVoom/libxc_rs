//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 771/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk771<F: Float>(t1165: F, t1188: F, t5867: F, t407: F, t5862: F, t1772: F, t301: F, t1089: F, t368: F, t1180: F, t1531: F, t3316: F, t418: F, t4450: F, t4524: F, t4532: F, t4538: F, t4558: F, t4561: F, t4563: F, t4565: F, t4603: F, t5842: F, t5844: F, t5846: F, t5848: F, t5850: F, t5855: F, t5859: F, t5864: F) -> (F, F, F, F, F) {
    let t5869 = t1165 * t5867 * t1188;
    let t5873 = t1165 * t5862 * t407;
    let t5876 = t1772 * t301;
    let t5878 = t1089 * t368 * t5876;
    let t5882 = F::cast_from(0.85748036236139473944e-3_f64) * t3316 + F::cast_from(0.85748036236139473944e-3_f64) * t4524 - t4532 + F::cast_from(0.80031500487063509016e-2_f64) * t4538 + F::cast_from(0.17149607247227894789e-2_f64) * t4558 - t4561 + t4563 - t4565 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t5842 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t5844 + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t5846 + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t5848 - F::cast_from(0.80031500487063509015e-2_f64) * t5850 - F::cast_from(0.12862205435420921092e-2_f64) * t4450 * t5855 + F::cast_from(0.12862205435420921092e-2_f64) * t1531 * t5859 - F::cast_from(0.42874018118069736972e-3_f64) * t1180 * t5864 + F::cast_from(0.42874018118069736972e-3_f64) * t1180 * t5869 - F::cast_from(0.21437009059034868486e-3_f64) * t1180 * t5873 - F::cast_from(0.17149607247227894789e-2_f64) * t418 * t5878 + F::cast_from(0.85748036236139473945e-2_f64) * t4603;
    (t5869, t5873, t5876, t5878, t5882)
}
