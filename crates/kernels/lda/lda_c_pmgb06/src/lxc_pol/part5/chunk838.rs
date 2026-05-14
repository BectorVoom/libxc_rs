//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 838/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk838<F: Float>(t12036: F, t500: F, t2010: F, t806: F, t1435: F, t1872: F, t132: F, t1547: F, t2107: F, t10203: F, t153: F, t1680: F, t2022: F, t2026: F, t2851: F, t814: F) -> (F, F, F, F, F, F, F, F) {
    let t12037 = t12036 * t500;
    let t12038 = 2.0 / 135.0 * t12037;
    let t12041 = t2010 * t806;
    let t12092 = t1435 * t1872;
    let t12112 = t132 * t1547 * t2107;
    let t12113 = t12112 / 45.0;
    let t12154 = t10203 * t153;
    let t12224 = t2022 * t1680;
    let t12225 = 2.0 / 9.0 * t12224;
    let t12227 = t2026 * t1680;
    let t12232 = t132 * t2851 * t814;
    (t12038, t12041, t12092, t12113, t12154, t12225, t12227, t12232)
}
