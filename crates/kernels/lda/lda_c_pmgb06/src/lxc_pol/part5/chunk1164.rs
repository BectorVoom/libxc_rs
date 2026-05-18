//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1164/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1164<F: Float>(t1420: F, t7542: F, t439: F, t5225: F, t7493: F, t1897: F, t19778: F, t443: F, t7801: F, t1385: F, t332: F, t5482: F, t6774: F) -> (F, F, F, F, F) {
    let t20981 = F::new(2.0) / F::new(15.0) * t1420 * t7542;
    let t20984 = F::new(2.0) / F::new(15.0) * t439 * t5225 * t7493;
    let t20987 = F::new(2.0) / F::new(15.0) * t439 * t1897 * t19778;
    let t20988 = t7801 * t443;
    let t20992 = t439 * t1385 * t20988 * t332 / F::new(45.0);
    let t20995 = t439 * t5482 * t6774 / F::new(15.0);
    (t20981, t20984, t20987, t20992, t20995)
}
