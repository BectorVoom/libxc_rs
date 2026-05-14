//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 665/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk665<F: Float>(t3194: F, t834: F, t1629: F, t1967: F, t1966: F, t1426: F, t2011: F, t1430: F, t1962: F, t1444: F, t1455: F, t1467: F, t1972: F, t1977: F, t1983: F, t2010: F, t2854: F, t2855: F, t2858: F, t3198: F, t439: F, t4585: F, t4589: F, t4593: F, t4602: F, t493: F, t835: F) -> (F, F, F, F, F, F) {
    let t4605 = t3194 * t834;
    let t4608 = t1967 * t1629;
    let t4609 = t1966 * t4608;
    let t4612 = t1426 * t2011;
    let t4615 = t1962 * t1430;
    let t4618 = t2854 - 2.0 / 135.0 * t2855 - 2.0 / 135.0 * t2858 + t493 * t4585 / 45.0 + t493 * t4589 / 27.0 + t4593 + t1972 * t1455 / 45.0 + t1972 * t1467 / 27.0 + t3198 * t835 / 45.0 + 2.0 / 45.0 * t1444 * t1977 - 4.0 / 45.0 * t4602 * t1983 + t493 * t4605 / 45.0 + t439 * t4609 / 15.0 + 4.0 / 45.0 * t2010 * t4612 + t439 * t4615 / 45.0;
    (t4605, t4608, t4609, t4612, t4615, t4618)
}
