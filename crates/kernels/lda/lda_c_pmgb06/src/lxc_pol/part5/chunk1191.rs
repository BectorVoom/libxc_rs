//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1191/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1191<F: Float>(t21375: F, t69: F, t21378: F, t11311: F, t11318: F, t21386: F, t21389: F, t21394: F, t21399: F, t2209: F, t2247: F, t2248: F, t2448: F, t5980: F, t769: F, t8263: F, t8287: F, t8295: F) -> F {
    let t21577 = t69 * t21375;
    let t21581 = t69 * t21378;
    let t21583 = F::new(15.518295) * t2247 * t2248 * t2209 * t2448 + F::new(15.518295) * t2247 * t2248 * t769 * t5980 + t21386 + t21389 + t8263 + t21399 + t8287 - t8295 + F::new(6.89702) * t21577 - F::new(1.724255) * t69 * t21394 + F::new(0.5747516666666667) * t21581 - t11311 + t11318;
    t21583
}
