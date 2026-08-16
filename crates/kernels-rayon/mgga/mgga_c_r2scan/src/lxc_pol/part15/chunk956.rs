//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 956/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk956(t10918: f64, t3275: f64, t3352: f64, t2289: f64, t3428: f64, t3430: f64, t10660: f64, t10665: f64, t10671: f64, t10678: f64, t10685: f64, t10690: f64, t10692: f64, t10695: f64, t10917: f64) -> (f64, f64, f64, f64) {
    let t10920 = t3275 * t10918 * t3352;
    let t10921 = t10920 / 2.0_f64;
    let t10922 = t2289 * t3428;
    let t10923 = t10922 * t3430;
    let t10924 = 0.15243824895787514157e-3_f64 * t10923;
    let t10925 = -0.30487649791575028314e-3_f64 * t10660 + t10665 - t10671 - 0.10248087766267884742e-3_f64 * t10678 + 0.72042316457491791906e-3_f64 * t10685 + t10690 - 0.36021158228745895953e-3_f64 * t10692 - 0.72042316457491791906e-3_f64 * t10695 - t10917 + t10921 + t10924;
    (t10921, t10922, t10924, t10925)
}
