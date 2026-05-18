//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1279/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1279<F: Float>(t1074: F, t6637: F, t5077: F, t5094: F, t1069: F, t13000: F, t5083: F, t1: F, t822: F, t332: F, t13043: F, t13047: F, t6646: F) -> (F, F, F, F, F, F, F, F) {
    let t16821 = t6637 * t1074;
    let t16824 = F::new(4.0) / F::new(45.0) * t5077 * t5094 * t16821;
    let t16825 = t6637 * t1069;
    let t16828 = F::new(4.0) / F::new(9.0) * t5083 * t13000 * t16825;
    let t16829 = t1 * t822;
    let t16830 = t16829 * t332;
    let t16833 = F::new(16.0) / F::new(45.0) * t13043 * t5094 * t16830;
    let t16835 = F::new(4.0) / F::new(27.0) * t13047 * t6646;
    (t16821, t16824, t16825, t16828, t16829, t16830, t16833, t16835)
}
