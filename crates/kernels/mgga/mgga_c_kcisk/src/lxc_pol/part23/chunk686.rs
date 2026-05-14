//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 686/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk686<F: Float>(t6100: F, t6101: F, t4011: F, t4063: F, t4108: F, t4115: F, t6020: F, t6023: F, t6026: F, t6030: F, t6044: F, t6052: F, t6060: F, t6062: F, t6066: F, t6069: F, t6072: F, t6076: F) -> (F, F) {
    let t6102 = t6100 * t6101;
    let t6119 = -0.1294625e1 * t6044 + 0.258925e1 * t6052 + t4108 + 0.10064166666666666667e0 * t4011 + 0.10064166666666666667e0 * t6020 - 0.20128333333333333333e0 * t6023 + 0.60385e0 * t6026 - 0.60385e0 * t6030 + 0.82524375e-1 * t6060 + 0.16504875e0 * t6062 + t4115 + 0.11038e0 * t4063 + 0.11038e0 * t6066 - 0.5519e-1 * t6069 + 0.33114e0 * t6072 - 0.33114e0 * t6076;
    (t6102, t6119)
}
