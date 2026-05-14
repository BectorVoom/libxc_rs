//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 666/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk666<F: Float>(t1435: F, t813: F, t1440: F, t439: F, t1423: F, t1969: F, t1431: F, t2002: F, t1887: F, t460: F, t1542: F, t802: F, t810: F, t947: F, t1860: F, t350: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4619 = t1435 * t813;
    let t4620 = t4619 * t1440;
    let t4622 = t439 * t4620 / 27.0;
    let t4624 = 4.0 / 45.0 * t1423 * t1969;
    let t4626 = t2002 * t1431 / 45.0;
    let t4628 = t1887 * t460 / 15.0;
    let t4630 = t802 * t1542 / 30.0;
    let t4635 = t947 * t810;
    let t4637 = t350 * t1860;
    (t4619, t4620, t4622, t4624, t4626, t4628, t4630, t4635, t4637)
}
