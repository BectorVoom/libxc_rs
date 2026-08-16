//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1115/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1115<F: Float>(t3295: F, t8018: F, t1577: F, t3308: F, t7438: F, t10710: F, t10768: F, t25737: F, t25499: F, t37586: F, t25503: F, t37658: F) -> (F, F, F, F, F) {
    let t39431 = t3295 * t8018;
    let t39434 = t1577 * t3308 * t7438;
    let t39437 = t10768 * t10710 * t25737;
    let t39440 = t37586 * t10710 * t25499;
    let t39443 = t37658 * t10710 * t25503;
    (t39431, t39434, t39437, t39440, t39443)
}
