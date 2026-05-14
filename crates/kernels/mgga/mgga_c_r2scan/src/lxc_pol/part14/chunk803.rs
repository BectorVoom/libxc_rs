//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 803/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk803<F: Float>(t6026: F, t7052: F, t7055: F, t7093: F, t7095: F, t7097: F, t7098: F, t7101: F, t7104: F, t7133: F, t7136: F, t765: F, t2055: F, t2056: F, t955: F, t2768: F, t761: F) -> (F, F, F) {
    let t7895 = 0.675260332e-1 * t765 * t7098 + 0.1350520664e0 * t765 * t7101 + 0.675260332e-1 * t765 * t7104 + 0.675260332e-1 * t765 * t7133 + 0.1350520664e0 * t765 * t7136 + t7052 - t7055 - t6026 - t7093 - t7095 + t7097;
    let t7898 = t2055 * t955 * t2056;
    let t7902 = t2768 * t761;
    (t7895, t7898, t7902)
}
