//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1149/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1149(t1773: f64, t2432: f64, t113: f64, t11674: f64, t11678: f64, t1289: f64, t1324: f64, t15086: f64, t15089: f64, t15096: f64, t15102: f64, t15106: f64, t15116: f64, t2255: f64, t2308: f64, t297: f64, t301: f64, t346: f64, t374: f64, t4358: f64, t5880: f64, t787: f64, t790: f64, t8092: f64, t8094: f64, t8097: f64, t8099: f64, t8105: f64, t8108: f64, t8163: f64) -> f64 {
    let t15121 = t1773 * t2432;
    let t15124 = -0.05321881782335382_f64 * t8092 - 0.31931290694012293_f64 * t8094 - t8097 + t8099 + t8105 - 1.82185769317151e-05_f64 * t8108 - 6.0_f64 * t4358 * t15086 + 24.0_f64 * t4358 * t15089 - 2.0_f64 * t346 * t2308 * t2255 * t374 - 2.0_f64 * t346 * t15096 * t1324 - 0.02394846802050922_f64 * t15102 - 0.01197423401025461_f64 * t15106 + 0.008135887625008338_f64 * t11674 - 0.013430671634934398_f64 * t11678 - t346 * t2308 * t787 * t1289 + t346 * t790 * t5880 - 0.01197423401025461_f64 * t297 * t15116 * t113 * t301 - 0.05321881782335382_f64 * t15121 - 0.02394846802050922_f64 * t8163;
    t15124
}
