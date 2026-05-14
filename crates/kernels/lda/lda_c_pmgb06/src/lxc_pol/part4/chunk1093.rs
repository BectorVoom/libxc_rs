//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1093/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1093<F: Float>(t513: F, t6688: F, t12447: F, t12449: F, t2002: F, t4780: F, t224: F, t6704: F, t446: F, t1427: F, t6127: F, t1989: F, t5305: F, t2493: F, t3213: F, t1963: F, t5187: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16338 = t6688 * t513 / 15.0;
    let t16339 = 4.0 / 135.0 * t12447;
    let t16340 = 4.0 / 135.0 * t12449;
    let t16342 = 4.0 / 45.0 * t2002 * t4780;
    let t16343 = t6704 * t224;
    let t16345 = 2.0 / 45.0 * t16343 * t446;
    let t16347 = 2.0 / 45.0 * t6127 * t1427;
    let t16349 = 4.0 / 45.0 * t5305 * t1989;
    let t16350 = t3213 * t2493;
    let t16351 = 4.0 / 405.0 * t16350;
    let t16353 = 4.0 / 45.0 * t5187 * t1963;
    (t16338, t16339, t16340, t16342, t16345, t16347, t16349, t16351, t16353)
}
