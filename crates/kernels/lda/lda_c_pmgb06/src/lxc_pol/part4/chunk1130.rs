//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1130/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1130<F: Float>(t4602: F, t6513: F, t1981: F, t5463: F, t6512: F, t1447: F, t6131: F, t1989: F, t5194: F, t2562: F, t607: F, t500: F, t1423: F, t6124: F, t439: F, t5197: F, t6555: F) -> (F, F, F, F, F, F, F) {
    let t16916 = 8.0 / 27.0 * t4602 * t6513;
    let t16919 = 8.0 / 27.0 * t1981 * t5463 * t6512;
    let t16920 = t1447 * t6131;
    let t16921 = 4.0 / 135.0 * t16920;
    let t16922 = t5194 * t1989;
    let t16923 = 8.0 / 135.0 * t16922;
    let t16924 = t2562 * t607;
    let t16925 = t16924 * t500;
    let t16926 = 4.0 / 135.0 * t16925;
    let t16927 = t1423 * t6124;
    let t16928 = 4.0 / 135.0 * t16927;
    let t16931 = 2.0 / 15.0 * t439 * t5197 * t6555;
    (t16916, t16919, t16921, t16923, t16926, t16928, t16931)
}
