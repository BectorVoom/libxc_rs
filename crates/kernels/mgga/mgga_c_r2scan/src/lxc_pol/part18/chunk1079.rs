//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1079/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1079<F: Float>(t1054: F, t6132: F, t8745: F, t6139: F, t8741: F, t40176: F, t40178: F, t40181: F, t40185: F, t43602: F, t43606: F, t43609: F, t43612: F, t43616: F, t43619: F, t39613: F, t40195: F, t8752: F) -> (F, F) {
    let t43622 = t6132 * t1054 * t8745;
    let t43625 = t6139 * t1054 * t8741;
    let t43627 = t40176 + 0.23115257973478049502e0 * t43602 + t40178 + t40181 - 0.31147743054556651236e-1 * t40185 + 0.21831846657716620896e-2 * t43606 - 0.46574606203128791245e-1 * t43609 - 0.26198215989259945075e-1 * t43612 + 0.27944763721877274747e0 * t43616 + 0.13002332610081402845e0 * t43619 - 0.17336443480108537126e0 * t43622 - 0.5200933044032561138e0 * t43625;
    let t43631 = t39613 * t40195 * t8752;
    (t43627, t43631)
}
