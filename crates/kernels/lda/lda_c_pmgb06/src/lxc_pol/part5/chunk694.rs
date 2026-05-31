//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 694/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk694<F: Float>(t5194: F, t835: F, t1447: F, t2462: F, t2466: F, t2470: F, t2527: F, t591: F, t2377: F, t330: F, t10: F, t2381: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6319 = t5194 * t835;
    let t6320 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t6319;
    let t6321 = t1447 * t2462;
    let t6322 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t6321;
    let t6323 = t1447 * t2466;
    let t6324 = F::cast_from(2.0_f64) / F::cast_from(135.0_f64) * t6323;
    let t6325 = t1447 * t2470;
    let t6326 = F::cast_from(2.0_f64) / F::cast_from(81.0_f64) * t6325;
    let t6327 = t2527 * t591;
    let t6329 = t330 * t2377;
    let t6334 = t10 * t2381;
    (t6319, t6320, t6321, t6322, t6323, t6324, t6325, t6326, t6327, t6329, t6334)
}
