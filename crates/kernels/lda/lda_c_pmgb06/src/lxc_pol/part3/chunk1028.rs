//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1028/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1028<F: Float>(t132: F, t137: F, t1629: F, t4815: F, t1680: F, t2022: F, t9457: F, t9461: F, t9467: F, t9470: F, t9478: F, t9481: F, t9483: F, t9491: F, t9494: F) -> (F, F) {
    let t12219 = t132 * t137 * t4815 * t1629 / F::new(10.0);
    let t12224 = t2022 * t1680;
    let t12225 = F::new(2.0) / F::new(9.0) * t12224;
    let t12226 = -t12219 + F::new(0.004546314527777778) * t9457 + t9461 + t9467 + t9470 + t9478 + t9481 + F::new(0.547) * t9483 + t9491 / F::new(3.0) + F::new(0.06077777777777778) * t9494 - t12225;
    (t12219, t12226)
}
