//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1039/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1039<F: Float>(t10781: F, t9513: F, t3308: F, t574: F, t9147: F, t1054: F, t2139: F, t8752: F, t2133: F, t8736: F, t40194: F, t40195: F, t8756: F, t39355: F, t39358: F, t39362: F, t39396: F, t39401: F, t39404: F, t39411: F) -> (F,) {
    let t43009 = t10781 * t9513;
    let t43012 = t574 * t3308 * t9147;
    let t43015 = t2139 * t1054 * t8752;
    let t43018 = t2133 * t1054 * t8736;
    let t43021 = t40194 * t40195 * t8756;
    let t43023 = -0.14282990759302185292e-1 * t39355 - 0.57131963037208741168e-1 * t39358 - 0.10975748638225852664e0 * t43009 - t39362 - 0.86682217400542685632e-1 * t43012 + 0.2600466522016280569e0 * t43015 + 0.86682217400542685632e-1 * t43018 - 0.32927245914677557992e0 * t43021 + t39396 - t39401 - t39404 + t39411;
    (t43023,)
}
