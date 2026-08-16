//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1091/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1091(t10659: f64, t10922: f64, t3428: f64, t3430: f64, t6809: f64, t6804: f64, t10943: f64, t10946: f64, t10928: f64, t122: f64, t166: f64, t2312: f64, t3434: f64) -> (f64, f64, f64, f64, f64) {
    let t38261 = t10922 * t10659;
    let t38264 = t6809 * t3428 * t3430;
    let t38267 = t6804 * t3428 * t3430;
    let t38269 = t10943 * t10946;
    let t38281 = t3434 * t10928 * t166 * t2312 * t122;
    (t38261, t38264, t38267, t38269, t38281)
}
