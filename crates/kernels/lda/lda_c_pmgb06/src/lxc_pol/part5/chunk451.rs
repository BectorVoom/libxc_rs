//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 451/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk451<F: Float>(t76: F, t769: F, t1246: F, t348: F, t773: F, t350: F, t342: F, t38: F, t776: F, t1212: F, t760: F, t1: F, t330: F) -> (F, F, F, F, F, F, F, F) {
    let t2181 = t76 * t769;
    let t2185 = F::cast_from(0.48717083333333333_f64) * t1246;
    let t2186 = t348 * t773;
    let t2187 = t2186 * t350;
    let t2188 = F::cast_from(0.48717083333333333_f64) * t2187;
    let t2191 = F::cast_from(5.84605_f64) * t38 * t776 * t342;
    let t2192 = t1212 * t760;
    let t2195 = t330 * t1;
    (t2181, t2185, t2186, t2187, t2188, t2191, t2192, t2195)
}
