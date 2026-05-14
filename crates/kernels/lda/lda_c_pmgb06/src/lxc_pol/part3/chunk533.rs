//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 533/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk533<F: Float>(t1382: F, t1447: F, t1600: F, t496: F, t1602: F, t507: F, t493: F, t1481: F, t529: F, t1380: F, t1640: F, t489: F, t161: F, t1641: F, t486: F, t1489: F, t517: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t2869 = t1447 * t1382;
    let t2870 = 4.0 / 45.0 * t2869;
    let t2871 = t496 * t1600;
    let t2872 = t507 * t1602;
    let t2873 = t2871 * t2872;
    let t2875 = 2.0 / 15.0 * t493 * t2873;
    let t2876 = t1481 * t529;
    let t2877 = t1380 * t2876;
    let t2879 = 2.0 / 15.0 * t493 * t2877;
    let t2880 = t489 * t1640;
    let t2881 = t161 * t2880;
    let t2882 = 2.0 / 15.0 * t2881;
    let t2884 = t486 * t1641 / 5.0;
    let t2885 = t1489 * t517;
    (t2869, t2870, t2871, t2872, t2873, t2875, t2876, t2877, t2879, t2880, t2881, t2882, t2884, t2885)
}
