//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1358/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1358<F: Float>(t13948: F, t13950: F, t13971: F, t17842: F, t17843: F, t17844: F, t17845: F, t17846: F, t17847: F, t17848: F, t17849: F, t17850: F, t17851: F, t17852: F, t17853: F) -> (F, F, F, F) {
    let t17854 = F::new(8.0) / F::new(405.0) * t13948;
    let t17855 = F::new(8.0) / F::new(135.0) * t13950;
    let t17856 = F::new(4.0) / F::new(45.0) * t13971;
    let t17857 = t17842 - t17843 - t17844 - t17845 - t17846 + t17847 - t17848 - t17849 + t17850 - t17851 - t17852 + t17853 + t17854 + t17855 - t17856;
    (t17854, t17855, t17856, t17857)
}
