//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 983/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk983<F: Float>(t38264: F, t3428: F, t3430: F, t6804: F, t10943: F, t10946: F, t10928: F, t122: F, t166: F, t2312: F, t3434: F, t10914: F, t3270: F, t2262: F, t4176: F, t1053: F, t10648: F, t10993: F, t6876: F) -> (F, F, F, F, F, F, F) {
    let t38265 = 0.15243824895787514157e-3 * t38264;
    let t38267 = t6804 * t3428 * t3430;
    let t38268 = 0.91462949374725084942e-3 * t38267;
    let t38269 = t10943 * t10946;
    let t38270 = 0.24390119833260022651e-2 * t38269;
    let t38281 = t3434 * t10928 * t166 * t2312 * t122;
    let t38282 = 0.65053455985619242968e-4 * t38281;
    let t38283 = t3270 * t10914;
    let t38288 = t4176 * t2262;
    let t38289 = t3270 * t38288;
    let t38297 = t10648 * t1053 * t6876 * t10993;
    (t38265, t38268, t38270, t38282, t38283, t38289, t38297)
}
