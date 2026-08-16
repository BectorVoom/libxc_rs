//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1203/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1203(t40428: f64, t10626: f64, t11479: f64, t3275: f64, t10935: f64, t2810: f64, t3446: f64, t37459: f64, t37461: f64, t37464: f64, t37468: f64, t40404: f64, t40406: f64, t40408: f64, t40411: f64, t40415: f64, t40419: f64, t40423: f64, t40426: f64) -> (f64, f64) {
    let t40429 = 0.10248087766267884742e-3_f64 * t40428;
    let t40432 = t3275 * t11479 * t10626 / 2.0_f64;
    let t40434 = t3446 * t10935 * t2810;
    let t40435 = 0.19211284388664477842e-2_f64 * t40434;
    let t40437 = t40404 - t40406 + t40408 - 0.72042316457491791906e-3_f64 * t40411 + t40415 + t40419 - t40423 + t37459 - t37461 - t37464 + t40426 - t40429 - t40432 + t40435 - 0.86737941314158990624e-4_f64 * t37468;
    (t40432, t40437)
}
