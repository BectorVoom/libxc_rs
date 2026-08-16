//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 609/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk609<F: Float>(t14567: F, t530: F, t14434: F, t570: F, t1356: F, t15030: F, t15033: F, t15037: F, t15041: F, t15044: F, t15047: F, t15062: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15419 = t530 * t14567;
    let t15420 = F::cast_from(0.2363e1_f64) * t15419;
    let t15421 = t14434 * t570;
    let t15422 = t1356 * t15421;
    let t15423 = F::cast_from(0.39914139006212695214e-1_f64) * t15422;
    let t15424 = F::cast_from(0.3252672799280962148e-5_f64) * t15030;
    let t15425 = F::cast_from(0.3252672799280962148e-5_f64) * t15033;
    let t15426 = F::cast_from(0.30487649791575028312e-3_f64) * t15037;
    let t15427 = F::cast_from(0.30487649791575028312e-3_f64) * t15041;
    let t15428 = F::cast_from(0.16263363996404810741e-4_f64) * t15044;
    let t15429 = F::cast_from(0.16263363996404810741e-4_f64) * t15047;
    let t15430 = F::cast_from(0.72042316457491791901e-3_f64) * t15062;
    (t15420, t15421, t15423, t15424, t15425, t15426, t15427, t15428, t15429, t15430)
}
