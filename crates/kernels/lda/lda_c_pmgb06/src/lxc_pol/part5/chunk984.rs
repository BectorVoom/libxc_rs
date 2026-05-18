//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 984/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk984<F: Float>(t1447: F, t6131: F, t1989: F, t5194: F, t2562: F, t607: F, t500: F, t1423: F, t6124: F, t1392: F, t2592: F, t2466: F, t3226: F) -> (F, F, F, F, F, F, F) {
    let t16920 = t1447 * t6131;
    let t16922 = t5194 * t1989;
    let t16924 = t2562 * t607;
    let t16925 = t16924 * t500;
    let t16927 = t1423 * t6124;
    let t16936 = t2592 * t1392;
    let t16962 = t3226 * t2466;
    (t16920, t16922, t16924, t16925, t16927, t16936, t16962)
}
