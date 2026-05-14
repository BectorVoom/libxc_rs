//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 535/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk535<F: Float>(t1163: F, t4372: F, t1530: F, t3371: F, t1535: F, t1162: F, t4180: F, t1016: F, t513: F, t1466: F, t3409: F, t1106: F, t1181: F, t540: F, t3391: F, t4198: F) -> (F, F, F, F, F, F, F, F) {
    let t4373 = t1163 * t4372;
    let t4389 = t1530 * t3371;
    let t4391 = 0.40015750243531754508e-2 * t4389 * t1535;
    let t4396 = t4180 * t1162;
    let t4398 = 0.85748036236139473944e-3 * t4396 * t1535;
    let t4417 = t1016 * t513;
    let t4423 = 0.40015750243531754508e-2 * t3409 * t1466;
    let t4425 = t1181 * t540 * t1106;
    let t4427 = 0.17149607247227894789e-2 * t3391 * t4425;
    let t4450 = t4198 * t1162;
    (t4373, t4391, t4398, t4417, t4423, t4425, t4427, t4450)
}
