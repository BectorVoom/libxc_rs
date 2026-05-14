//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 995/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk995<F: Float>(t2480: F, t439: F, t4779: F, t1992: F, t493: F, t529: F, t7806: F, t9636: F, t2088: F, t6285: F, t1966: F, t477: F, t7811: F, t9647: F, t1444: F, t7660: F) -> (F, F, F, F, F) {
    let t20627 = t439 * t4779 * t2480 / 15.0;
    let t20632 = 4.0 / 5.0 * t493 * t1992 * t9636 * t7806 * t529;
    let t20636 = 3.0 / 5.0 * t493 * t1992 * t6285 * t2088;
    let t20641 = 4.0 / 5.0 * t439 * t1966 * t9647 * t7811 * t477;
    let t20643 = 2.0 / 9.0 * t1444 * t7660;
    (t20627, t20632, t20636, t20641, t20643)
}
