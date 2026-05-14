//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 993/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk993<F: Float>(t1380: F, t2088: F, t2549: F, t493: F, t17283: F, t17285: F, t17287: F, t1966: F, t2064: F, t439: F, t6253: F, t2462: F, t5194: F, t16924: F, t835: F, t16382: F, t806: F) -> (F, F, F, F, F, F, F, F) {
    let t20601 = t493 * t1380 * t2549 * t2088 / 15.0;
    let t20602 = 2.0 / 45.0 * t17283;
    let t20603 = 4.0 / 45.0 * t17285;
    let t20604 = 2.0 / 27.0 * t17287;
    let t20608 = 3.0 / 5.0 * t439 * t1966 * t6253 * t2064;
    let t20609 = t5194 * t2462;
    let t20610 = 4.0 / 45.0 * t20609;
    let t20611 = t16924 * t835;
    let t20612 = 2.0 / 45.0 * t20611;
    let t20613 = t16382 * t806;
    (t20601, t20602, t20603, t20604, t20608, t20610, t20612, t20613)
}
