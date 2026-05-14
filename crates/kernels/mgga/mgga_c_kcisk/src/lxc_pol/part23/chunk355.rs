//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 355/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk355<F: Float>(t1161: F, t1184: F, t1165: F, t1176: F, t1181: F, t1188: F) -> (F, F, F) {
    let t1531 = 0.516475e0 * t1161;
    let t1534 = 0.104195e0 * t1184;
    let t1536 = 0.3529725e1 * t1176 - t1531 - 0.516475e0 * t1165 + 0.6311625e0 * t1181 - t1534 - 0.104195e0 * t1188;
    (t1531, t1534, t1536)
}
