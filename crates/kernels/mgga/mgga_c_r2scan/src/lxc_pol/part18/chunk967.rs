//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 967/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk967<F: Float>(t38244: F, t38241: F, t6855: F, t269: F, t597: F, t10650: F, t10655: F, t10659: F, t10922: F, t3428: F, t3430: F, t6809: F, t6804: F, t10943: F, t10946: F, t10928: F, t122: F, t166: F, t2312: F, t3434: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t38245 = 0.10260057759007034251e-5 * t38244;
    let t38248 = t6855 * t38241;
    let t38249 = t597 * t269;
    let t38251 = t38248 * t10650 * t38249;
    let t38259 = t10655 * t10659;
    let t38261 = t10922 * t10659;
    let t38264 = t6809 * t3428 * t3430;
    let t38265 = 0.15243824895787514157e-3 * t38264;
    let t38267 = t6804 * t3428 * t3430;
    let t38268 = 0.91462949374725084942e-3 * t38267;
    let t38269 = t10943 * t10946;
    let t38270 = 0.24390119833260022651e-2 * t38269;
    let t38281 = t3434 * t10928 * t166 * t2312 * t122;
    (t38245, t38248, t38249, t38251, t38259, t38261, t38265, t38268, t38270, t38281)
}
