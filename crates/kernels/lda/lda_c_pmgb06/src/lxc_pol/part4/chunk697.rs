//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 697/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk697<F: Float>(t1195: F, t4194: F, t115: F, t1180: F, t562: F, t1669: F, t1194: F, t113: F, t247: F, t2801: F, t1139: F, t199: F) -> (F, F, F, F, F, F, F, F) {
    let t4196 = F::new(0.02358774) * t4194 * t1195;
    let t4197 = t1180 * t115;
    let t4199 = F::cast_from(0.09753333333333333_f64) * t562 * t4197;
    let t4200 = t1669 * t115;
    let t4202 = F::new(0.03145032) * t1194 * t4200;
    let t4205 = F::cast_from(0.001883059277350998_f64) * t113 * t247 * t115;
    let t4208 = F::new(12.0) * t2801;
    let t4212 = t1139 * t199;
    (t4196, t4197, t4199, t4200, t4202, t4205, t4208, t4212)
}
