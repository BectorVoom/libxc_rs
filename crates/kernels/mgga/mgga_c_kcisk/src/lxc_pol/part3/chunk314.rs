//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 314/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk314<F: Float>(t1536: F, t1537: F, t1203: F, t325: F, t1210: F, t1212: F, t240: F, t1169: F, t1194: F, t1198: F, t1213: F, t1524: F, t1529: F, t516: F, t547: F, t524: F) -> (F, F, F, F, F, F, F, F) {
    let t1538 = t1536 * t1537;
    let t1542 = t325 * t1203;
    let t1543 = t1210 * t1212;
    let t1550 = t240 * t325;
    let t1553 = -t1169 + t1194 + t240 * (-0.3109e-1 * t1524 * t516 + 1.0 * t1529 * t1538 + t1169 - t1194 - 0.19751789702565206229e-1 * t1198 + 0.58482233974552040708e0 * t1542 * t1543) + 0.19751789702565206229e-1 * t240 * t1198 - 0.58482233974552040708e0 * t1550 * t1213;
    let t1555 = t547 * t547;
    let t1556 = 1.0 / t1555;
    let t1557 = t524 * t1556;
    (t1538, t1542, t1543, t1550, t1553, t1555, t1556, t1557)
}
