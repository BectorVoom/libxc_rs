//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 792/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk792<F: Float>(t140: F, t3737: F, t4594: F, t5056: F, t5049: F, t5074: F, t139: F, t172: F, t79: F, t721: F, t4805: F, t4811: F, t1865: F, t3805: F, t4582: F, t4799: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10494 = t140 * t3737 * t4594;
    let t10495 = t10494 * t5056;
    let t10497 = t5074 * t5049;
    let t10500 = t139 * t172 * t79;
    let t10501 = t10500 * t721;
    let t10502 = 0.73697530864197530862e-3 * t10501;
    let t10515 = t4811 * t4805;
    let t10517 = t3805 * t1865;
    let t10527 = t4811 * t4582;
    let t10532 = t4811 * t4799;
    (t10494, t10495, t10497, t10500, t10501, t10502, t10515, t10517, t10527, t10532)
}
