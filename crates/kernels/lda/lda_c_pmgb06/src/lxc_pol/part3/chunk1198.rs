//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1198/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1198<F: Float>(t14347: F, t1377: F, t2345: F, t97: F, t27: F, t545: F, t5635: F, t5638: F, t5632: F, t12230: F, t12233: F, t12235: F, t12237: F, t12240: F, t12242: F, t12244: F) -> F {
    let t14348 = F::new(0.03354522822333102) * t14347;
    let t14350 = t2345 * t97 * t1377;
    let t14353 = t5635 * t27 * t545;
    let t14356 = t5638 * t27 * t545;
    let t14357 = F::new(0.6492624817418906) * t14356;
    let t14359 = t5632 * t27 * t545;
    let t14361 = t14348 + F::new(0.03354522822333102) * t14350 + F::new(0.3246312408709453) * t14353 + t14357 + F::new(0.3246312408709453) * t14359 + t12230 + t12233 + t12235 - t12237 + t12240 + t12242 - t12244;
    t14361
}
