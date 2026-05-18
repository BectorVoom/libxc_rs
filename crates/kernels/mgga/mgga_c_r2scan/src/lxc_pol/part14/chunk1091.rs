//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1091/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1091<F: Float>(t10659: F, t10922: F, t3428: F, t3430: F, t6809: F, t6804: F, t10943: F, t10946: F, t10928: F, t122: F, t166: F, t2312: F, t3434: F) -> (F, F, F, F, F) {
    let t38261 = t10922 * t10659;
    let t38264 = t6809 * t3428 * t3430;
    let t38267 = t6804 * t3428 * t3430;
    let t38269 = t10943 * t10946;
    let t38281 = t3434 * t10928 * t166 * t2312 * t122;
    (t38261, t38264, t38267, t38269, t38281)
}
