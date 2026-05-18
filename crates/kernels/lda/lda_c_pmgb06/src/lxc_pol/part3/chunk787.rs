//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 787/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk787<F: Float>(t44: F, t5430: F, t131: F, t178: F, t1848: F, t513: F, t1491: F, t831: F, t1512: F, t815: F, t1831: F, t529: F) -> (F, F, F, F, F, F, F) {
    let t5431 = t5430 * t44;
    let t5432 = t5431 * t131;
    let t5434 = t5432 * t178 / F::new(30.0);
    let t5436 = t1848 * t513 / F::new(15.0);
    let t5438 = t831 * t1491 / F::new(30.0);
    let t5440 = t1512 * t815 / F::new(30.0);
    let t5441 = t1831 * t529;
    (t5431, t5432, t5434, t5436, t5438, t5440, t5441)
}
