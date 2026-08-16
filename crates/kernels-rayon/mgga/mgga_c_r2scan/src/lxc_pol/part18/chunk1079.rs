//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1079/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1079(t38264: f64, t3428: f64, t3430: f64, t6804: f64, t10943: f64, t10946: f64, t10928: f64, t122: f64, t166: f64, t2312: f64, t3434: f64, t1053: f64, t10648: f64, t10993: f64, t6876: f64) -> (f64, f64, f64, f64, f64) {
    let t38265 = 0.15243824895787514157e-3_f64 * t38264;
    let t38267 = t6804 * t3428 * t3430;
    let t38268 = 0.91462949374725084942e-3_f64 * t38267;
    let t38269 = t10943 * t10946;
    let t38270 = 0.24390119833260022651e-2_f64 * t38269;
    let t38281 = t3434 * t10928 * t166 * t2312 * t122;
    let t38282 = 0.65053455985619242968e-4_f64 * t38281;
    let t38297 = t10648 * t1053 * t6876 * t10993;
    (t38265, t38268, t38270, t38282, t38297)
}
