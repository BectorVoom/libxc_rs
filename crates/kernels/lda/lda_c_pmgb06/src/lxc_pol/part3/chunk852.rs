//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 852/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk852<F: Float>(t3620: F, t377: F, t1289: F, t1295: F, t3631: F, t374: F, t3630: F, t67: F, t73: F, t2786: F, t56: F, t69: F) -> (F, F, F, F, F) {
    let t8396 = t3620 * t377;
    let t8399 = t1289 * t1295;
    let t8404 = t374 * t3631;
    let t8413 = t67 / t3630 / t73;
    let t8428 = F::cast_from(2.9801938271604937_f64) * t69 * t2786 * t56;
    (t8396, t8399, t8404, t8413, t8428)
}
