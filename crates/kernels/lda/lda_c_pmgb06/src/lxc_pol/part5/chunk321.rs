//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 321/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk321<F: Float>(t1193: F, t98: F, t115: F, t569: F, t1072: F, t1105: F, t247: F, t290: F, t395: F, t701: F, t250: F) -> (F, F, F, F, F, F, F, F) {
    let t1194 = t1193 * t98;
    let t1195 = t569 * t115;
    let t1197 = F::new(0.00786258) * t1194 * t1195;
    let t1198 = F::new(4.0) * t1072;
    let t1199 = F::new(3.0) * t1105;
    let t1205 = F::new(0.31995040645307626) * t247 * t290;
    let t1206 = t395 * t701;
    let t1212 = F::new(1.0) / t250;
    (t1194, t1195, t1197, t1198, t1199, t1205, t1206, t1212)
}
