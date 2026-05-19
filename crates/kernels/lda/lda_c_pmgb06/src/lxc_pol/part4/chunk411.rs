//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 411/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk411<F: Float>(t5: F, t1504: F, t161: F, t1069: F, t1074: F, t44: F, t131: F, zeta_threshold: F) -> (F, F, F, F) {
    let t6 = t5 <= zeta_threshold;
    let t1505 = t161 * t1504;
    let t1506 = F::new(2.0) / F::new(45.0) * t1505;
    let t1510 = piecewise3::<F>(t6, F::new(0.0), F::new(2.0) * t5 * t1074 + F::new(2.0) * t1069);
    let t1511 = t1510 * t44;
    let t1512 = t1511 * t131;
    (t1505, t1506, t1511, t1512)
}
