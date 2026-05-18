//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 728/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk728<F: Float>(t1911: F, t5486: F, t493: F, t176: F, t1988: F, t1826: F, t4588: F, t1821: F, t2549: F, t529: F, t1380: F, t1414: F, t2389: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6744 = t5486 * t1911;
    let t6746 = F::new(2.0) / F::new(45.0) * t493 * t6744;
    let t6747 = t1988 * t176;
    let t6748 = t6747 * t1826;
    let t6750 = F::new(4.0) / F::new(45.0) * t493 * t6748;
    let t6751 = t4588 * t176;
    let t6752 = t6751 * t1821;
    let t6754 = F::new(2.0) / F::new(27.0) * t493 * t6752;
    let t6755 = t2549 * t529;
    let t6756 = t1380 * t6755;
    let t6758 = t493 * t6756 / F::new(45.0);
    let t6759 = t1414 * t2389;
    (t6744, t6746, t6747, t6748, t6750, t6751, t6752, t6754, t6755, t6756, t6758, t6759)
}
