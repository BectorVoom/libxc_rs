//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 592/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk592<F: Float>(t168: F, t2782: F, t286: F, t142: F, t1568: F, t1724: F, t454: F, t1549: F, t1734: F, t1704: F, t1554: F, t455: F, t1553: F, t450: F, t1555: F, t1729: F, t452: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t2783 = t168 * t2782;
    let t2785 = 0.19513566535229734 * t2783 * t286;
    let t2786 = t142 * t1568;
    let t2790 = t454 * t1724;
    let t2791 = t2790 * t142;
    let t2793 = t1549 * t1734;
    let t2798 = t142 * t1704;
    let t2799 = t1554 * t2798;
    let t2801 = t455 * t2786;
    let t2805 = t1553 * t450;
    let t2806 = t2805 * t1555;
    let t2809 = t1729 * t452;
    (t2783, t2785, t2786, t2790, t2791, t2793, t2798, t2799, t2801, t2805, t2806, t2809)
}
