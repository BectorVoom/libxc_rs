//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 371/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk371<F: Float>(t1696: F, t83: F, t208: F, t213: F, t580: F, t97: F, t588: F, t604: F, t607: F, t109: F, t131: F) -> (F, F, F, F, F, F, F) {
    let t1697 = t83 * t1696;
    let t1698 = t1697 * t208;
    let t1700 = t1698 * t213 / F::new(3.0);
    let t1701 = t580 * t97;
    let t1703 = F::new(0.12155555555555556) * t1701 * t588;
    let t1708 = t604 * t607;
    let t1710 = t131 * t109;
    (t1697, t1698, t1700, t1701, t1703, t1708, t1710)
}
