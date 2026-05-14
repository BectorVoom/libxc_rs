//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 499/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk499<F: Float>(t5: F, t76: F, t769: F, t1246: F, t348: F, t773: F, t350: F, t342: F, t38: F, t776: F, t1212: F, t760: F, t1: F, t330: F, t332: F, t395: F, t1219: F, t764: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6 = t5 <= zeta_threshold;
    let t2181 = t76 * t769;
    let t2185 = 0.48717083333333333 * t1246;
    let t2186 = t348 * t773;
    let t2187 = t2186 * t350;
    let t2188 = 0.48717083333333333 * t2187;
    let t2191 = 5.84605 * t38 * t776 * t342;
    let t2192 = t1212 * t760;
    let t2195 = t330 * t1;
    let t2199 = piecewise3(t6, 0.0, -2.0 / 9.0 * t2192 * t332 + 4.0 / 3.0 * t2195 * t395);
    let t2200 = t1219 * t764;
    (t2181, t2185, t2186, t2188, t2191, t2192, t2195, t2199, t2200)
}
