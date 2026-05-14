//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 993/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk993<F: Float>(t13507: F, t1995: F, t3198: F, t1444: F, t5176: F, t5319: F, t3226: F, t3284: F, t493: F, t6119: F, t5180: F, t1489: F, t165: F, t1994: F, t1588: F, t1848: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13508 = 2.0 / 27.0 * t13507;
    let t13510 = t3198 * t1995 / 5.0;
    let t13512 = 2.0 / 5.0 * t1444 * t5176;
    let t13514 = t1444 * t5319 / 5.0;
    let t13515 = t3226 * t1995;
    let t13516 = 4.0 / 15.0 * t13515;
    let t13519 = t493 * t6119 * t3284 / 5.0;
    let t13521 = 2.0 / 5.0 * t1444 * t5180;
    let t13525 = t493 * t165 * t1489 * t1994 / 5.0;
    let t13527 = t1848 * t1588 / 10.0;
    (t13508, t13510, t13512, t13514, t13516, t13519, t13521, t13525, t13527)
}
