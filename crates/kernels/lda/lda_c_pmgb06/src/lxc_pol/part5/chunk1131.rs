//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1131/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1131<F: Float>(t493: F, t6503: F, t6751: F, t1981: F, t6406: F, t6747: F, t1444: F, t7509: F, t2979: F, t7508: F, t1380: F, t6827: F, t851: F) -> (F, F, F, F, F) {
    let t20584 = F::new(2.0) / F::new(3.0) * t493 * t6751 * t6503;
    let t20587 = F::new(8.0) / F::new(15.0) * t1981 * t6747 * t6406;
    let t20589 = t1444 * t7509 / F::new(15.0);
    let t20592 = t493 * t2979 * t7508 / F::new(15.0);
    let t20596 = t493 * t1380 * t6827 * t851 / F::new(15.0);
    (t20584, t20587, t20589, t20592, t20596)
}
