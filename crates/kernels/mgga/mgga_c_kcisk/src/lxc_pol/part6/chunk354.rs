//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 354/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk354<F: Float>(t1459: F, t2282: F, t1522: F, t2077: F, t1531: F, t1534: F, t2084: F, t2087: F, t2090: F, t1537: F, t1212: F, t2105: F, t1529: F, t1542: F, t1550: F, t2081: F, t2095: F, t2098: F, t2107: F, t240: F, t516: F) -> (F, F, F, F, F, F) {
    let t2283 = t1459 * t2282;
    let t2285 = -t1522 - 0.17123333333333333333e-1 * t2077;
    let t2292 = 0.3529725e1 * t2084 - t1531 - 0.516475e0 * t2077 + 0.6311625e0 * t2087 - t1534 - 0.104195e0 * t2090;
    let t2293 = t2292 * t1537;
    let t2297 = t2105 * t1212;
    let t2306 = -t2081 + t2095 + t240 * (-0.3109e-1 * t2285 * t516 + 1.0 * t1529 * t2293 + t2081 - t2095 - 0.19751789702565206229e-1 * t2098 + 0.58482233974552040708e0 * t1542 * t2297) + 0.19751789702565206229e-1 * t240 * t2098 - 0.58482233974552040708e0 * t1550 * t2107;
    (t2283, t2285, t2292, t2293, t2297, t2306)
}
