//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 852/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk852<F: Float>(t360: F, t8357: F, t3566: F, t8305: F, t3631: F, t374: F, t3630: F, t67: F, t73: F, t2786: F, t56: F, t69: F) -> (F, F, F, F, F) {
    let t8358 = t360 * t8357;
    let t8388 = t3566 * t8305;
    let t8404 = t374 * t3631;
    let t8413 = t67 / t3630 / t73;
    let t8428 = F::new(2.9801938271604937) * t69 * t2786 * t56;
    (t8358, t8388, t8404, t8413, t8428)
}
