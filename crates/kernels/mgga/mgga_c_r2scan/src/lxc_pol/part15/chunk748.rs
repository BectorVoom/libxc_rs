//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 748/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk748<F: Float>(t7109: F, t1419: F, t899: F, t7055: F, t7058: F, t7091: F, t7093: F, t7095: F, t7097: F, t7098: F, t7101: F, t7104: F, t7108: F, t881: F, t2266: F, t6890: F, t910: F) -> (F, F, F, F) {
    let t7110 = 20.0 * t7109;
    let t7111 = t1419 * t899;
    let t7112 = 12.0 * t7111;
    let t7113 = -t7055 - t7058 - t7091 - t7093 - t7095 + t7097 - 0.2363e1 * t881 * t7098 - 0.4726e1 * t881 * t7101 - 0.2363e1 * t881 * t7104 + t7108 - t7110 - t7112;
    let t7116 = t2266 * t6890 * t910;
    (t7110, t7112, t7113, t7116)
}
