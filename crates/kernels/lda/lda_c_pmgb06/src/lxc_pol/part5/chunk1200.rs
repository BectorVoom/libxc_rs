//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1200/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1200<F: Float>(t5: F, t1068: F, t1072: F, t19870: F, t2125: F, t21326: F, t332: F, t4486: F, t4489: F, t5961: F, t6042: F, t6698: F, t7284: F, t7290: F, t8485: F, t9: F, zeta_threshold: F) -> F {
    let t6 = t5 <= zeta_threshold;
    let t21750 = piecewise3::<F>(t6, F::new(0.0), F::new(40.0) / F::new(81.0) * t8485 * t7284 * t332 - F::new(16.0) / F::new(9.0) * t6042 * t1072 - F::new(8.0) / F::new(9.0) * t4486 * t6698 + F::new(8.0) / F::new(3.0) * t4489 * t21326 + F::new(4.0) / F::new(3.0) * t2125 * t5961 + F::new(4.0) / F::new(9.0) * t1068 * t7290 * t332 + F::new(4.0) / F::new(3.0) * t9 * t19870);
    t21750
}
