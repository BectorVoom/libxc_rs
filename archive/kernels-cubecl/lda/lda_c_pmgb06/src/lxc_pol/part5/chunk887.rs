//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 887/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk887<F: Float>(t135: F, t1438: F, t144: F, t3247: F, t511: F, t1464: F, t164: F, t170: F, t3259: F, t458: F, t1426: F, t1592: F) -> (F, F, F, F, F) {
    let t10203 = F::cast_from(1.0_f64) / t135 / t1438 * t144;
    let t10220 = t3247 * t511;
    let t10230 = F::cast_from(1.0_f64) / t164 / t1464 * t170;
    let t10247 = t3259 * t458;
    let t10288 = t1426 * t1592;
    (t10203, t10220, t10230, t10247, t10288)
}
