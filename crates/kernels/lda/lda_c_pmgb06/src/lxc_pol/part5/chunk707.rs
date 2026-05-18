//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 707/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk707<F: Float>(t1901: F, t6146: F, t439: F, t5260: F, t6151: F, t6155: F, t2010: F, t1916: F, t1972: F, t1920: F, t1894: F, t2002: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6472 = t1901 * t6146;
    let t6474 = F::new(2.0) / F::new(9.0) * t439 * t6472;
    let t6475 = t5260 * t6151;
    let t6477 = F::new(8.0) / F::new(81.0) * t439 * t6475;
    let t6478 = t1901 * t6155;
    let t6480 = F::new(4.0) / F::new(27.0) * t2010 * t6478;
    let t6482 = F::new(4.0) / F::new(45.0) * t1972 * t1916;
    let t6484 = F::new(2.0) / F::new(27.0) * t1972 * t1920;
    let t6486 = F::new(2.0) / F::new(45.0) * t2002 * t1894;
    (t6472, t6474, t6475, t6477, t6478, t6480, t6482, t6484, t6486)
}
