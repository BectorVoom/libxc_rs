//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 965/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk965<F: Float>(t132: F, t1547: F, t2107: F, t10247: F, t153: F, t10203: F, t435: F, t5119: F, t1447: F, t5282: F, t1680: F, t2022: F, t2026: F, t2851: F, t814: F, t2852: F, t802: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12112 = t132 * t1547 * t2107;
    let t12139 = t10247 * t153;
    let t12154 = t10203 * t153;
    let t12191 = t132 * t435 * t5119;
    let t12202 = t1447 * t5282;
    let t12224 = t2022 * t1680;
    let t12227 = t2026 * t1680;
    let t12232 = t132 * t2851 * t814;
    let t12234 = t802 * t2852;
    (t12112, t12139, t12154, t12191, t12202, t12224, t12227, t12232, t12234)
}
