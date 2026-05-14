//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 540/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk540<F: Float>(t3330: F, t119: F, t2983: F, t1041: F, t1067: F, t1025: F, t1062: F, t721: F, t4093: F, t633: F, t903: F, t1106: F, t3223: F, t1040: F, t1098: F, t1007: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4406 = 0.10237773105191754 * t3330;
    let t4411 = t119 * t2983;
    let t4413 = t1041 * t1067;
    let t4420 = t1025 * t1062;
    let t4421 = t4420 * t721;
    let t4426 = t903 * t4093 * t633;
    let t4429 = t1106 * t3223;
    let t4437 = t1040 * t1062;
    let t4438 = t4437 * t721;
    let t4440 = t1040 * t119;
    let t4445 = t1098 * t3223;
    let t4449 = t1007 * t1067;
    (t4406, t4411, t4413, t4421, t4426, t4429, t4438, t4440, t4445, t4449)
}
