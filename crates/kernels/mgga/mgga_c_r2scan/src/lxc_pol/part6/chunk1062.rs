//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1062/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1062<F: Float>(t374: F, t8505: F, t2449: F, t885: F, t1044: F, t1353: F, t1039: F, t2337: F, t915: F, t95: F, t3270: F, t983: F, t571: F, t7983: F, t1569: F, t2530: F) -> (F, F, F, F, F, F, F, F) {
    let t8506 = t8505 * t374;
    let t8507 = t2449 * t885;
    let t8509 = t1353 * t1044;
    let t8517 = t1039 * t2337;
    let t8668 = t915 * t95;
    let t8707 = t3270 * t983;
    let t8792 = t571 * t7983;
    let t9507 = t1569 * t2530;
    (t8506, t8507, t8509, t8517, t8668, t8707, t8792, t9507)
}
