//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 720/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk720<F: Float>(t1593: F, t443: F, t760: F, t822: F, t477: F, t5077: F, t332: F, t5094: F, t5084: F, t5083: F, t2563: F, t513: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6636 = t1593 * t443;
    let t6637 = t760 * t822;
    let t6638 = t6637 * t477;
    let t6639 = t6636 * t6638;
    let t6641 = F::new(4.0) / F::new(45.0) * t5077 * t6639;
    let t6642 = t6637 * t332;
    let t6643 = t5094 * t6642;
    let t6645 = F::new(4.0) / F::new(45.0) * t5077 * t6643;
    let t6646 = t5084 * t6642;
    let t6648 = F::new(2.0) / F::new(27.0) * t5083 * t6646;
    let t6650 = t2563 * t513 / F::new(30.0);
    (t6636, t6637, t6639, t6641, t6643, t6645, t6646, t6648, t6650)
}
