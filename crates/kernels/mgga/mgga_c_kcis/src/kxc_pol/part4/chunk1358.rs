//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1358/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1358<F: Float>(t17463: F, t17464: F, t15898: F, t4293: F, t4292: F, t12534: F, t251: F, t15929: F, t5903: F, t1532: F, t1929: F, t4262: F) -> (F, F, F, F) {
    let t17465 = t17463 * t17464;
    let t17467 = t4293 * t15898;
    let t17468 = t4292 * t17467;
    let t17470 = t251 * t12534;
    let t17471 = t17470 * t15929;
    let t17472 = t5903 * t17471;
    let t17474 = t1532 * t1929;
    let t17475 = t17474 * t4262;
    (t17465, t17468, t17472, t17475)
}
