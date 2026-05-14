//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1021/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1021<F: Float>(t1773: F, t2432: F, t113: F, t11674: F, t11678: F, t1289: F, t1324: F, t15086: F, t15089: F, t15096: F, t15102: F, t15106: F, t15116: F, t2255: F, t2308: F, t297: F, t301: F, t346: F, t374: F, t4358: F, t5880: F, t787: F, t790: F, t8092: F, t8094: F, t8097: F, t8099: F, t8105: F, t8108: F, t8163: F) -> (F,) {
    let t15121 = t1773 * t2432;
    let t15124 = -0.05321881782335382 * t8092 - 0.31931290694012293 * t8094 - t8097 + t8099 + t8105 - 1.82185769317151e-05 * t8108 - 6.0 * t4358 * t15086 + 24.0 * t4358 * t15089 - 2.0 * t346 * t2308 * t2255 * t374 - 2.0 * t346 * t15096 * t1324 - 0.02394846802050922 * t15102 - 0.01197423401025461 * t15106 + 0.008135887625008338 * t11674 - 0.013430671634934398 * t11678 - t346 * t2308 * t787 * t1289 + t346 * t790 * t5880 - 0.01197423401025461 * t297 * t15116 * t113 * t301 - 0.05321881782335382 * t15121 - 0.02394846802050922 * t8163;
    (t15124,)
}
