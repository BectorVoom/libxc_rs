//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1090/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1090(t38244: f64, t38241: f64, t6855: f64, t269: f64, t597: f64, t10650: f64, t10655: f64, t10659: f64, t10922: f64, t3428: f64, t3430: f64, t6809: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t38245 = 0.10260057759007034251e-5_f64 * t38244;
    let t38248 = t6855 * t38241;
    let t38249 = t597 * t269;
    let t38251 = t38248 * t10650 * t38249;
    let t38259 = t10655 * t10659;
    let t38261 = t10922 * t10659;
    let t38264 = t6809 * t3428 * t3430;
    (t38245, t38248, t38249, t38251, t38259, t38261, t38264)
}
