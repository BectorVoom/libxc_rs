//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1010/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1010<F: Float>(t19395: F, t493: F, t496: F, t498: F, t2470: F, t5305: F, t1972: F, t6282: F, t1988: F, t6544: F, t1444: F, t7634: F, t1420: F, t7532: F, t12772: F, t2500: F, t439: F) -> (F, F, F, F, F, F, F) {
    let t20870 = t493 * t496 * t498 * t19395 / 45.0;
    let t20872 = t5305 * t2470 / 9.0;
    let t20874 = t1972 * t6282 / 9.0;
    let t20877 = t493 * t1988 * t6544 / 15.0;
    let t20879 = t1444 * t7634 / 9.0;
    let t20881 = 2.0 / 15.0 * t1420 * t7532;
    let t20884 = 2.0 / 15.0 * t439 * t12772 * t2500;
    (t20870, t20872, t20874, t20877, t20879, t20881, t20884)
}
