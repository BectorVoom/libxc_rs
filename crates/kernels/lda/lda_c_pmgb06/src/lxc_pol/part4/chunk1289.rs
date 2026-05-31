//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1289/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1289<F: Float>(t16936: F, t479: F, t6705: F, t1397: F, t2592: F, t13192: F, t13194: F, t1444: F, t6120: F, t16916: F, t16919: F, t16921: F, t16923: F, t16926: F, t16928: F, t16931: F, t16933: F, t16935: F) -> (F, F, F, F, F, F, F) {
    let t16937 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t16936;
    let t16939 = t6705 * t479 / F::cast_from(15.0_f64);
    let t16941 = t2592 * t1397 / F::cast_from(15.0_f64);
    let t16942 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t13192;
    let t16943 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t13194;
    let t16945 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t1444 * t6120;
    let t16946 = -t16916 - t16919 + t16921 + t16923 + t16926 + t16928 + t16931 - t16933 - t16935 - t16937 - t16939 - t16941 - t16942 - t16943 + t16945;
    (t16937, t16939, t16941, t16942, t16943, t16945, t16946)
}
