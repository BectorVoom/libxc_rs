//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 881/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk881<F: Float>(t432: F, t6621: F, t1554: F, t161: F, t2653: F, t1848: F, t2018: F, t1447: F, t6131: F, t1989: F, t5194: F, t2562: F, t607: F, t500: F, t1423: F, t6124: F) -> (F, F, F, F, F, F, F, F) {
    let t16877 = t432 * t6621;
    let t16880 = t161 * t1554 * t2653;
    let t16884 = t1848 * t2018;
    let t16920 = t1447 * t6131;
    let t16922 = t5194 * t1989;
    let t16924 = t2562 * t607;
    let t16925 = t16924 * t500;
    let t16927 = t1423 * t6124;
    (t16877, t16880, t16884, t16920, t16922, t16924, t16925, t16927)
}
