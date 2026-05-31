//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1203/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1203<F: Float>(t40428: F, t10626: F, t11479: F, t3275: F, t10935: F, t2810: F, t3446: F, t37459: F, t37461: F, t37464: F, t37468: F, t40404: F, t40406: F, t40408: F, t40411: F, t40415: F, t40419: F, t40423: F, t40426: F) -> (F, F) {
    let t40429 = F::cast_from(0.10248087766267884742e-3_f64) * t40428;
    let t40432 = t3275 * t11479 * t10626 / F::cast_from(2.0_f64);
    let t40434 = t3446 * t10935 * t2810;
    let t40435 = F::cast_from(0.19211284388664477842e-2_f64) * t40434;
    let t40437 = t40404 - t40406 + t40408 - F::cast_from(0.72042316457491791906e-3_f64) * t40411 + t40415 + t40419 - t40423 + t37459 - t37461 - t37464 + t40426 - t40429 - t40432 + t40435 - F::cast_from(0.86737941314158990624e-4_f64) * t37468;
    (t40432, t40437)
}
