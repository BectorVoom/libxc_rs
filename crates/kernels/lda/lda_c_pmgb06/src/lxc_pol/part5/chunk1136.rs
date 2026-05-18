//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1136/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1136<F: Float>(t1444: F, t7660: F, t3238: F, t493: F, t7659: F, t7567: F, t10220: F, t7566: F, t439: F, t6550: F, t6555: F, t2481: F, t5187: F) -> (F, F, F, F, F, F) {
    let t20643 = F::new(2.0) / F::new(9.0) * t1444 * t7660;
    let t20646 = F::new(2.0) / F::new(9.0) * t493 * t3238 * t7659;
    let t20648 = F::new(8.0) / F::new(81.0) * t1444 * t7567;
    let t20651 = F::new(8.0) / F::new(81.0) * t493 * t10220 * t7566;
    let t20654 = t439 * t6550 * t6555 / F::new(5.0);
    let t20656 = t5187 * t2481 / F::new(15.0);
    (t20643, t20646, t20648, t20651, t20654, t20656)
}
