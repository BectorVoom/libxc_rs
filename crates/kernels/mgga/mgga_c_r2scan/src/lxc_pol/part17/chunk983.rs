//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 983/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk983<F: Float>(t11199: F, t3262: F, t3574: F, t2867: F, t3275: F, t3270: F, t3719: F, t3269: F, t10678: F, t10685: F, t11580: F, t11585: F, t11589: F, t11593: F, t11598: F, t11601: F, t11604: F, t11607: F, t12054: F, t12059: F, t12062: F) -> (F, F, F, F, F) {
    let t12081 = t3262 * t11199 * t3574;
    let t12082 = F::new(3.0) / F::new(4.0) * t12081;
    let t12084 = t3275 * t11199 * t2867;
    let t12085 = t12084 / F::new(4.0);
    let t12086 = t3270 * t3719;
    let t12087 = t3269 * t12086;
    let t12088 = t12087 / F::new(4.0);
    let t12089 = -t12054 + F::cast_from(0.1921128438866447784e-2_f64) * t11580 + F::cast_from(0.72042316457491791901e-3_f64) * t11585 - t12059 - t12062 + F::cast_from(0.72042316457491791901e-3_f64) * t11589 - F::cast_from(0.10248087766267884741e-3_f64) * t11593 - F::cast_from(0.10248087766267884741e-3_f64) * t10678 + F::cast_from(0.72042316457491791901e-3_f64) * t10685 - F::cast_from(0.72042316457491791901e-3_f64) * t11598 - F::cast_from(0.72042316457491791901e-3_f64) * t11601 - F::cast_from(0.30487649791575028312e-3_f64) * t11604 - F::cast_from(0.72042316457491791901e-3_f64) * t11607 - t12082 + t12085 - t12088;
    (t12081, t12084, t12086, t12087, t12089)
}
