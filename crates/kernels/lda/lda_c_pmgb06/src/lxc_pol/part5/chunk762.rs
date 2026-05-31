//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 762/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk762<F: Float>(t4786: F, t4788: F, t4792: F, t4794: F, t4807: F, t4809: F, t4812: F, t4814: F, t4950: F, t4970: F, t5633: F, t5640: F, t6427: F, t6428: F, t6429: F) -> F {
    let t7190 = t4786 + t4788 + t4792 + t4794 + t4807 + t4809 + t4812 + t4814 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t5633 + t5640 + t6427 + t6428 - t4950 - t4970 - t6429;
    t7190
}
