//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 338/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk338<F: Float>(t1193: F, t98: F, t115: F, t569: F, t1072: F, t1105: F, t1185: F, t1189: F, t1192: F, t81: F) -> (F, F, F, F, F) {
    let t1194 = t1193 * t98;
    let t1195 = t569 * t115;
    let t1197 = F::cast_from(0.00786258_f64) * t1194 * t1195;
    let t1198 = F::cast_from(4.0_f64) * t1072;
    let t1199 = F::cast_from(3.0_f64) * t1105;
    let t1200 = t1185 + t1189 - t1192 + t1197 + t81 - t1198 + t1199;
    (t1194, t1195, t1197, t1199, t1200)
}
