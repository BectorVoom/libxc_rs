//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 983/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk983(t11199: f64, t3262: f64, t3574: f64, t2867: f64, t3275: f64, t3270: f64, t3719: f64, t3269: f64, t10678: f64, t10685: f64, t11580: f64, t11585: f64, t11589: f64, t11593: f64, t11598: f64, t11601: f64, t11604: f64, t11607: f64, t12054: f64, t12059: f64, t12062: f64) -> (f64, f64, f64, f64, f64) {
    let t12081 = t3262 * t11199 * t3574;
    let t12082 = 3.0_f64 / 4.0_f64 * t12081;
    let t12084 = t3275 * t11199 * t2867;
    let t12085 = t12084 / 4.0_f64;
    let t12086 = t3270 * t3719;
    let t12087 = t3269 * t12086;
    let t12088 = t12087 / 4.0_f64;
    let t12089 = -t12054 + 0.1921128438866447784e-2_f64 * t11580 + 0.72042316457491791901e-3_f64 * t11585 - t12059 - t12062 + 0.72042316457491791901e-3_f64 * t11589 - 0.10248087766267884741e-3_f64 * t11593 - 0.10248087766267884741e-3_f64 * t10678 + 0.72042316457491791901e-3_f64 * t10685 - 0.72042316457491791901e-3_f64 * t11598 - 0.72042316457491791901e-3_f64 * t11601 - 0.30487649791575028312e-3_f64 * t11604 - 0.72042316457491791901e-3_f64 * t11607 - t12082 + t12085 - t12088;
    (t12081, t12084, t12086, t12087, t12089)
}
