//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 484/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk484<F: Float>(t12: F, t15: F, t2386: F, t2389: F, t598: F, t2510: F, t44: F, t1929: F, t1931: F, t1934: F, t1936: F, t205: F, t2414: F, t208: F, t1998: F, t1679: F, t1682: F, t1700: F, t1703: F, t1939: F, t213: F, t224: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13 = t12 <= zeta_threshold;
    let t2516 = piecewise3(t13, 0.0, 40.0 / 9.0 * t15 * t2386 + 8.0 / 3.0 * t598 * t2389);
    let t2519 = (t2510 / 2.0 + t2516 / 2.0) * t44;
    let t2522 = 2.0 / 45.0 * t1929;
    let t2523 = 2.0 / 45.0 * t1931;
    let t2524 = 2.0 / 45.0 * t1934;
    let t2525 = 2.0 / 45.0 * t1936;
    let t2526 = t2414 * t205;
    let t2527 = t2526 * t208;
    let t2531 = 4.0 / 135.0 * t1998;
    let t2532 = t1679 - t1682 + t1700 + t1703 - t2519 * t224 / 15.0 + t2522 + t2523 + t2524 + t2525 + t2527 * t213 / 3.0 - 4.0 / 45.0 * t1939 + t2531;
    (t2519, t2522, t2523, t2524, t2525, t2526, t2527, t2531, t2532)
}
