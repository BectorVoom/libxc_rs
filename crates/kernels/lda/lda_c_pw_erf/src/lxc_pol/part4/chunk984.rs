//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 984/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk984<F: Float>(t3166: F, t749: F, t1784: F, t343: F, t1792: F, t1765: F, t2948: F, t1077: F, t4393: F, t344: F, t4405: F, t1064: F, t1799: F, t390: F, t40: F, t4383: F) -> (F, F, F, F, F, F, F, F) {
    let t11405 = t3166 * t749;
    let t11430 = 32.0 * t1784 * t343;
    let t11456 = 32.0 * t1792 * t343;
    let t11463 = t1765 * t2948;
    let t11465 = t4393 * t1077;
    let t11469 = t344 * t4405;
    let t11471 = t1064 * t1799;
    let t11474 = t40 * t4383 * t390;
    (t11405, t11430, t11456, t11463, t11465, t11469, t11471, t11474)
}
