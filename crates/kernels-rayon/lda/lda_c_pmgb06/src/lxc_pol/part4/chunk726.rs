//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 726/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk726(t3194: f64, t834: f64, t1629: f64, t1967: f64, t1966: f64, t1426: f64, t2011: f64, t1430: f64, t1962: f64, t1444: f64, t1455: f64, t1467: f64, t1972: f64, t1977: f64, t1983: f64, t2010: f64, t2854: f64, t2855: f64, t2858: f64, t3198: f64, t439: f64, t4585: f64, t4589: f64, t4593: f64, t4602: f64, t493: f64, t835: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4605 = t3194 * t834;
    let t4608 = t1967 * t1629;
    let t4609 = t1966 * t4608;
    let t4612 = t1426 * t2011;
    let t4615 = t1962 * t1430;
    let t4618 = t2854 - 2.0_f64 / 135.0_f64 * t2855 - 2.0_f64 / 135.0_f64 * t2858 + t493 * t4585 / 45.0_f64 + t493 * t4589 / 27.0_f64 + t4593 + t1972 * t1455 / 45.0_f64 + t1972 * t1467 / 27.0_f64 + t3198 * t835 / 45.0_f64 + 2.0_f64 / 45.0_f64 * t1444 * t1977 - 4.0_f64 / 45.0_f64 * t4602 * t1983 + t493 * t4605 / 45.0_f64 + t439 * t4609 / 15.0_f64 + 4.0_f64 / 45.0_f64 * t2010 * t4612 + t439 * t4615 / 45.0_f64;
    (t4605, t4608, t4609, t4612, t4615, t4618)
}
