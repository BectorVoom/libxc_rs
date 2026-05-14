//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 848/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk848<F: Float>(t2849: F, t749: F, t3160: F, t3166: F, t1765: F, t2948: F, t1077: F, t4393: F, t344: F, t4405: F, t1064: F, t1799: F, t285: F, t4422: F, t477: F, t1128: F, t1896: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11401 = t2849 * t749;
    let t11403 = t3160 * t749;
    let t11404 = 240.0 * t11403;
    let t11405 = t3166 * t749;
    let t11463 = t1765 * t2948;
    let t11465 = t4393 * t1077;
    let t11466 = 3.5089340384731225 * t11465;
    let t11469 = t344 * t4405;
    let t11471 = t1064 * t1799;
    let t11472 = 60.0 * t11471;
    let t11498 = t4422 * t477 * t285;
    let t11499 = 0.0017434044910732151 * t11498;
    let t11501 = t1896 * t1128 * t285;
    (t11401, t11404, t11405, t11463, t11466, t11469, t11472, t11499, t11501)
}
